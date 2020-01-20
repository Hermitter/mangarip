use crate::scrape::Scraper;
use crate::Error;
use url::Url;

#[derive(PartialEq, Debug)]
pub enum Sorting {
    Ascending,
    Descending,
}

pub fn find(url: &str) -> Result<Scraper, Error> {
    match Url::parse(url).unwrap().host_str().unwrap() {
        "mangakakalot.com" => Ok(Scraper {
            chapter_css_selector: ".chapter-list .row span a",
            image_css_selector: "#vungdoc img",
            chapter_sort: Sorting::Descending,
            url: url,
            chapter_urls: Vec::new(),
        }),
        "mangairo.com" => Ok(Scraper {
            chapter_css_selector: "",
            image_css_selector: "",
            chapter_sort: Sorting::Ascending,
            url: url,
            chapter_urls: Vec::new(),
        }),
        "manganelo.com" => Ok(Scraper {
            chapter_css_selector: "",
            image_css_selector: "",
            chapter_sort: Sorting::Ascending,
            url: url,
            chapter_urls: Vec::new(),
        }),
        _ => Err(Error::UnsupportedHost {
            url: url.to_string(),
        }),
    }
}
