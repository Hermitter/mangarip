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
    /// Create a book with a url to each chapter from the host site.
    pub async fn new<'b>(url: &str, host: &'a Host<'_>) -> Result<Book<'a>, Error> {
        Ok(Book {
            host,
            url: url.to_owned(),
            chapters: host.get_chapters(url).await?,
        })
    }

    /// Populate each chapter with the URL to every image inside.
    pub async fn scan(&mut self) {
        let mut futures = Vec::new();

        for chapter in self.chapters.iter_mut() {
            futures.push(chapter.get_image_urls(&self.host.image_selector));
        }

        join_all(futures).await;
    }
}
