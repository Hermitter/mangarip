mod standard;
use super::error::ScrapeError;
pub use standard::Scraper;

pub trait Fetch {
    /// Update self with each chapter's url
    // fn scan_toc(&mut self) -> Result<(), ScrapeError>;

    /// Fetches each image and calls closure for each chapter scraped
    fn get_chapter(&self, chapter_number: u32) -> Result<Vec<Vec<u8>>, ScrapeError>;

    /// Scrape chapters from start to finish. Calls closure on each successful scrape.
    fn get_chapters<F: Fn(u32)>(
        &self,
        start: u32,
        finish: u32,
        f: F,
    ) -> Result<Vec<Vec<Vec<u8>>>, ScrapeError>;
}
