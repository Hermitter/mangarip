mod simple;
use super::error::ScrapeError;
pub use simple::Scraper;

pub trait Fetch {
    /// Update self with each chapter's url
    fn scan_toc(&mut self) -> Result<(), ScrapeError>;

    /// Fetches each image and calls closure for each chapter scraped
    // fn for_each_chapter<F: Fn()>(&self, f: F) -> Result<Vec<Vec<u8>>, ScrapeError>;

    fn get_chapter(&self, chapter_number: u32) -> Result<Vec<Vec<u8>>, ScrapeError>;

    // fn chapters(&self) -> Result<Vec<String>, ScrapeError>;
}
