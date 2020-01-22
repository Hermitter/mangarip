use crate::scrape::Scraper;
use crate::Error;
use url::Url;

/// Describes how the table of contents is sorted.
/// This will tell us if it's by newest or oldest first.
#[derive(PartialEq, Debug)]
pub enum Sorting {
    Ascending,
    Descending,
}

/// Returns a pre-made Scrapper for a supported website.
pub fn find(url: &str) -> Result<Scraper, Error> {
    // Breakdown:
    // - `src`:         "src"
    // - ` *= *`:       "=" surrounded by zero or more spaces
    // - `"(...)"`:     capture only what's inside quotes
    // - `[^"]+`:       anything that's not a quote, one or more times
    // - `/\d+`:        URL separator followed one or more digits (0-9)
    // - `\.`:          an escaped single dot
    // - `(?:jpg|png)`: a non-capture group (?:) with either "jpg" or "png"
    let image_regex = regex::bytes::Regex::new(r#"src *= *"([^"]+/\d+\.(?:jpg|png))""#).unwrap();

    match Url::parse(url).unwrap().host_str().unwrap() {
        "mangakakalot.com" => Ok(Scraper {
            chapter_css_selector: ".chapter-list .row span a",
            chapter_sort: Sorting::Descending,
            url,
            chapter_urls: Vec::new(),
            image_regex,
            chapter_closure: None,
        }),
        "mangairo.com" => Ok(Scraper {
            chapter_css_selector: "",
            chapter_sort: Sorting::Ascending,
            url,
            chapter_urls: Vec::new(),
            image_regex,
            chapter_closure: None,
        }),
        "manganelo.com" => Ok(Scraper {
            chapter_css_selector: "",
            chapter_sort: Sorting::Ascending,
            url,
            chapter_urls: Vec::new(),
            image_regex,
            chapter_closure: None,
        }),
        _ => Err(Error::UnsupportedHost {
            url: url.to_owned(),
        }),
    }
}
