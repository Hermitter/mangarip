mod host;
mod toc;
use crate::Error;
pub use host::Host;
use toc::TableOfContents;

#[derive(Debug)]
pub struct Chapter {
    /// Each image from a chapter.
    pages: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct Book {
    /// Table of contents for a manga. This will
    pub toc: TableOfContents,

    /// Endpoint for all downloaded chapters.
    pub chapters: Vec<Chapter>,
}

impl Book {
    /// Create an instance of Book with the table of contents already scanned.
    pub async fn from(url: &str, host: &Host) -> Result<Book, Error> {
        let mut book = Book {
            toc: TableOfContents::new(url, host),
            chapters: Vec::new(),
        };

        book.toc.scan().await?;
        Ok(book)
    }

    /// Download every image(page) for a specific chapter
    pub async fn download_chapter(&self, index: u32) -> Result<Chapter, Error> {
        if index > self.toc.chapter_urls.len() as u32 {
            return Err(Error::InvalidChapter { index });
        }

        let mut chapter = Chapter { pages: Vec::new() };

        Ok(chapter)
    }
}
