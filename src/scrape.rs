use crate::{error, host, web_util};
use kuchiki;
use kuchiki::traits::*;

#[derive(PartialEq)]
pub enum Sorting {
    Ascending,
    Descending,
}

// All the information needed to support a new website
pub struct Scraper<'a> {
    pub chapter_sort: Sorting,
    // each chapter href in table of contents
    pub chapter_css_selector: &'a str,
    // each img src in chapter
    pub image_css_selector: &'a str,
    pub url: &'a str,
}

impl<'a> Scraper<'a> {
    // Check if host is supported and return a Scrape with host's configuration
    pub fn from(url: &str) -> Result<Scraper, error::ScrapeError> {
        host::find(url)
    }

    // returns a vector of links to each chapter in a manga
    pub fn get_chapters(&self) -> Result<Vec<String>, error::ScrapeError> {
        let mut chapters = Vec::new();

        // download table of contents HTML
        let document = kuchiki::parse_html()
            .one(web_util::get_html(self.url))
            .select(self.chapter_css_selector);

        // check if HTML can be parsed
        if !document.is_ok() {
            return Err(error::ScrapeError::UnreadableHTML(self.url.to_string()));
        }

        // store link for each chapter
        for css_match in document.unwrap() {
            let node = css_match.as_node();
            let a = node.as_element().unwrap().attributes.borrow();
            let href = a.get("href").unwrap();

            chapters.push(href.to_string());
        }

        // convert descending list to ascending
        if self.chapter_sort == Sorting::Descending {
            &mut chapters.reverse();
        }

        Ok(chapters)
    }

    // pub fn get_images() {
    //     // store image link for each chapter
    //     let mut result = Vec::<Vec<u8>>::new();

    //     for mut chapter_url in &mut chapters {
    //         web_util::get_html(&chapter_url);

    //         let document = kuchiki::parse_html().one(web_util::get_html(&chapter_url));

    //         // store image from each chapter
    //         for css_match in document.select(self.image_selector).unwrap() {
    //             let node = css_match.as_node();
    //             let img = node.as_element().unwrap().attributes.borrow();
    //             let src = img.get("src").unwrap();

    //             result.push(web_util::get_bytes(src).unwrap());
    //         }
    //     }

    //     result
    // }
}
