pub mod chapter;
use chapter::Chapter;
mod fetch;
mod host;
use crate::Error;
use futures::future::join_all;
pub use host::Host;

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

    /// Update the URL for each chapter found in the Host's table of contents.
    pub async fn scan_chapters<'b>(mut self) -> Result<Book<'a>, Error> {
        self.chapters = self.host.get_chapters(&self.url).await?;
        Ok(self)
    }

    /// Populate each chapter with a URL to each image it contains.
    pub async fn scan_images(mut self) -> Book<'a> {
        let mut futures = Vec::new();

        for chapter in self.chapters.iter_mut() {
            // TODO: set chapter vec to be Option<Vec<String>> so that errors can be scanned for.
            futures.push(chapter.get_image_urls(&self.host.image_selector));
        }

        join_all(futures).await;

        self
    }
}
