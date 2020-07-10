mod chapter;
mod host;
mod page;
use crate::lib::{Error, Selector};
pub use chapter::Chapter;
use futures::future::try_join_all;
pub use host::Host;

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
    /// Create an instance of `Book` with a url to each chapter.
    pub async fn from<'b>(url: &str, host: &'a Host) -> Result<Book<'a>, Error> {
        Ok(Book {
            host,
            url: url.to_owned(),
            chapters: host.scan(url).await?,
        })
    }

    // /// Populate each `Page` for a specific `Chapter`.
    // pub async fn download_chapter(&mut self, index: usize) -> Result<(), Error> {
    //     self.chapters[index]
    //         .borrow_mut()
    //         .download(&self.host.page_selector)
    //         .await?;

    //     Ok(())
    // }

    // /// Populate each `Page` for a range of `Chapter`s.
    // pub async fn download_chapters(&mut self, start: u32, end: u32) -> Result<(), Error> {
    //     // function to download the images of a chapter
    //     async fn get_chapter(chapter: &RefCell<Chapter>, selector: &Selector) -> Result<(), Error> {
    //         chapter.borrow_mut().download(selector).await
    //     }

    //     let mut futures = vec![];
    //     for i in start..end {
    //         futures.push(get_chapter(
    //             &self.chapters[i as usize],
    //             &self.host.page_selector,
    //         ));
    //     }

    //     try_join_all(futures).await?;

    //     Ok(())
    // }

    // /// Populate each `Page` of every `Chapter` in a `Book`.
    // pub async fn download_all_chapters(&mut self) -> Result<(), Error> {
    //     self.download_chapters(0, (self.chapters.len() - 1) as u32)
    //         .await?;
    //     Ok(())
    // }
}
