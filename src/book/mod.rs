pub mod chapter;
use chapter::Chapter;
mod fetch;
mod host;
use crate::Error;
pub use host::Host;
use std::sync::Arc;
use tokio::task::spawn;

#[derive(PartialEq, Debug, Clone)]
pub enum Selector {
    Regex(String),
    Css(String),
}

/// Describes how a numbered list is sorted.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Sorting {
    Ascending,
    Descending,
}

#[derive(Debug)]
pub struct Book<'a> {
    /// Information needed to support the site that's hosting the book.
    pub host: &'a Host<'a>,
    /// URL to the table of contents of a manga.
    pub url: String,
    /// Each chapter of a manga.
    pub chapters: Vec<Chapter>,
}

impl<'a> Book<'a> {
    /// Create a book to store information on chapters and their images.
    pub fn new<'b>(url: &str, host: &'a Host<'_>) -> Book<'a> {
        Book {
            host,
            url: url.to_owned(),
            chapters: vec![],
        }
    }

    /// Find every available chapter for the current book.
    pub async fn scan_chapters<'b>(mut self) -> Result<Book<'a>, Error> {
        self.chapters = self.host.get_chapters(&self.url).await?;
        Ok(self)
    }

    /// Find every available image for each chapter.
    pub async fn scan_images(mut self) -> Book<'a> {
        let selector = Arc::new(self.host.image_selector.clone());
        let mut handles = vec![];
        let num_of_chapters = self.chapters.len();

        for chapter in self.chapters.into_iter() {
            let selector_arc = selector.clone();

            handles.push(spawn(async move {
                chapter.scan_images(&selector_arc).await.unwrap() // TODO handle errors
            }));
        }

        self.chapters = Vec::new();
        self.chapters.reserve(num_of_chapters);

        for handle in handles {
            self.chapters.push(handle.await.unwrap());
        }

        self
    }
}
