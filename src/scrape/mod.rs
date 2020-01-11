mod simple;

pub use simple::Scraper;

use super::error::ScrapeError;

pub trait Fetch {
    fn chapter_urls(&self) -> Result<Vec<String>, ScrapeError>;

    // fn get_images(&self) -> Result<Vec<String>, ScrapeError>;
}
