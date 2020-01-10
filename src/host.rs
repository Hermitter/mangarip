use crate::scrape::{Scraper, Sorting};
use crate::ScrapeError;
use url::Url;

pub fn find(url: &str) -> Result<Scraper, ScrapeError> {
    match Url::parse(url).unwrap().host_str().unwrap() {
        "mangakakalot.com" => Ok(Scraper {
            chapter_css_selector: ".chapter-list .row span a",
            image_css_selector: "#vungdoc img",
            chapter_sort: Sorting::Descending,
            url: url,
        }),
        "mangairo.com" => Ok(Scraper {
            chapter_css_selector: "",
            image_css_selector: "",
            chapter_sort: Sorting::Ascending,
            url: url,
        }),
        "manganelo.com" => Ok(Scraper {
            chapter_css_selector: "",
            image_css_selector: "",
            chapter_sort: Sorting::Ascending,
            url: url,
        }),
        _ => Err(ScrapeError::UnsupportedHost(url.to_string())),
    }
}
