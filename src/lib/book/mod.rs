pub mod chapter;
pub mod host;
use crate::lib::Error;
use chapter::Chapter;
use futures::future::join_all;
use host::Host;

#[derive(Debug)]
pub struct Book<'a> {
    /// Information needed to support the site a book is hosted on.
    pub host: &'a Host,
    /// URL to the table of contents of a manga.
    pub url: String,
    /// Each chapter of a manga.
    pub chapters: Vec<Chapter>,
}

impl<'a> Book<'a> {
    /// Create a book with a url to each chapter from the host site.
    pub async fn new<'b>(url: &str, host: &'a Host) -> Result<Book<'a>, Error> {
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
