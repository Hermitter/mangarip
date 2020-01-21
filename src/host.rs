use crate::scrape::Scraper;
use crate::Error;
use url::Url;

#[derive(PartialEq, Debug)]
pub enum Sorting {
    Ascending,
    Descending,
}

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
            url: url,
            chapter_urls: Vec::new(),
            image_regex,
        }),
        "mangairo.com" => Ok(Scraper {
            chapter_css_selector: "",
            chapter_sort: Sorting::Ascending,
            url: url,
            chapter_urls: Vec::new(),
            image_regex,
        }),
        "manganelo.com" => Ok(Scraper {
            chapter_css_selector: "",
            chapter_sort: Sorting::Ascending,
            url: url,
            chapter_urls: Vec::new(),
            image_regex,
        }),
        _ => Err(Error::UnsupportedHost {
            url: url.to_owned(),
        }),
    }
}
