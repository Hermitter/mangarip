use super::web_utils;
use kuchiki;
use kuchiki::traits::*;

#[derive(PartialEq)]
pub enum Sorting {
    Ascending,
    Descending,
}

struct Chapter {
    image: String,
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
    pub fn scrape(&self, url: &str) {
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
                image: href.to_string(),
            });
        }

        // convert descending list to ascending
        if self.chapter_sort == Sorting::Descending {
            &mut chapters.reverse();
        }

        // store each image link for each chapter
        for mut chapter in chapters {
            web_utils::get_html(&chapter.image);

            let document = kuchiki::parse_html().one(web_utils::get_html(&chapter.image));

            // store image from each chapter
            for css_match in document.select(self.image_selector).unwrap() {
                let node = css_match.as_node();
                let img = node.as_element().unwrap().attributes.borrow();
                let src = img.get("src").unwrap();

                // chapter.image = src.to_string();
                println!("{}", src);
            }
        }
    }
}
