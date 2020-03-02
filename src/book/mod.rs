mod chapter;
mod host;
mod page;
use crate::Error;
pub use chapter::Chapter;
use futures::future::try_join_all;
pub use host::Host;
use page::Page;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Book<'a> {
    /// Information needed to support a book's host site.
    pub host: &'a Host,
    /// URL to the table of contents of a manga.
    pub url: String,
    /// Endpoint for all downloaded chapters.
    pub chapters: Vec<Chapter>,
}

impl<'a> Book<'a> {
    /// Create an instance of Book with the amount of chapters and
    pub async fn from<'b>(url: &str, host: &'a Host) -> Result<Book<'a>, Error> {
        Ok(Book {
            host,
            url: url.to_owned(),
            chapters: host.scan(url).await?,
        })
    }

    pub async fn download_chapter(&mut self, index: usize) -> Result<(), Error> {
        let chapter = &mut self.chapters[index];
        chapter.scan(&self.host.page_selector).await.unwrap();

        // function to download the page and update `Page.content` with it.
        async fn get_image(page: &mut RefCell<Page>) -> Result<(), Error> {
            page.borrow_mut().download().await
        }

        let mut futures = vec![];
        for page in &mut chapter.pages {
            futures.push(get_image(page));
        }

        try_join_all(futures).await?;

        Ok(())
    }
}
