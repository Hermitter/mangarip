use super::web_utils;
use kuchiki;
use kuchiki::traits::*;

#[derive(PartialEq)]
pub enum Sorting {
    Ascending,
    Descending,
}

pub struct Chapter {
    pub image_url: String,
    pub image_bytes: Vec<u8>,
}

// All the information needed to support a new website
pub struct Scraper<'a> {
    // CSS selector for each chapter in table of contents
    pub chapter_selector: &'a str,
    pub chapter_sort: Sorting,
    // CSS selector for each chapter's image
    pub image_selector: &'a str,
}

impl<'a> Scraper<'a> {
    pub fn scrape(&self, url: &str) -> Vec<Chapter> {
        // links for each chapter
        let mut chapters = Vec::<Chapter>::new();

        // download table of contents HTML
        let document = kuchiki::parse_html().one(web_utils::get_html(url));

        // store link for each chapter
        for css_match in document.select(self.chapter_selector).unwrap() {
            let node = css_match.as_node();
            let a = node.as_element().unwrap().attributes.borrow();
            let href = a.get("href").unwrap();

            chapters.push(Chapter {
                image_url: href.to_string(),
                image_bytes: Vec::new(),
            });
        }

        // convert descending list to ascending
        if self.chapter_sort == Sorting::Descending {
            &mut chapters.reverse();
        }

        // store image link for each chapter
        for mut chapter in &mut chapters {
            web_utils::get_html(&chapter.image_url);

            let document = kuchiki::parse_html().one(web_utils::get_html(&chapter.image_url));

            // store image from each chapter
            for css_match in document.select(self.image_selector).unwrap() {
                let node = css_match.as_node();
                let img = node.as_element().unwrap().attributes.borrow();
                let src = img.get("src").unwrap();

                chapter.image_bytes = web_utils::get_bytes(src).unwrap();
            }
        }

        chapters
    }
}
