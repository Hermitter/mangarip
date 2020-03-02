mod chapter;
mod host;
mod page;
use crate::Error;
pub use chapter::Chapter;
use futures::future::try_join_all;
pub use host::Host;
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

    pub async fn download_chapter(&mut self, index: i32) {
        let chapter = &mut self.chapters[0];
        chapter.scan(&self.host.page_selector).await.unwrap();

        // for page in chapter.pages.iter() {
        //     println!("{:?}", page);
        // }

        for page in chapter.pages.split_at_mut() {
            println!("--> {:?}", page[0]);
        }

        // wait for all images to download
        // try_join_all(chapter.pages.into_iter().map(|x| x.download())).await;
    }
}
