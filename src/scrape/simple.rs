use crate::{error::ScrapeError, host, scrape::Fetch, web_util};
use kuchiki;
use kuchiki::traits::*;
use std::thread;

/// All the information needed to support a new website
#[derive(Debug)]
pub struct Scraper<'a> {
    /// url to the table of contents
    pub url: &'a str,

    /// urls to each chapter in the manga
    pub chapter_urls: Option<Vec<String>>,

    /// how the table of contents is sorted
    pub chapter_sort: host::Sorting,

    /// selector to each chapter url in the table of contents
    pub chapter_css_selector: &'a str,

    /// selector to each image url in a chapter
    pub image_css_selector: &'a str,
}

impl<'a> Scraper<'a> {
    /// Check if host is supported and return a Scrape with host's configuration
    pub fn from(url: &str) -> Result<Scraper, ScrapeError> {
        host::find(url)
    }
}

impl<'a> Fetch for Scraper<'a> {
    fn scan_toc(&mut self) -> Result<(), ScrapeError> {
        let mut urls = vec![];

        // download table of contents HTML
        let document = kuchiki::parse_html()
            .one(web_util::get_html(self.url)?.as_string()?)
            .select(self.chapter_css_selector);

        // check if HTML can be parsed
        if !document.is_ok() {
            return Err(ScrapeError::UnreadableHtml { url: self.url });
        }

        // store link for each chapter
        for css_match in document? {
            let node = css_match.as_node();
            let a = node.as_element().unwrap().attributes.borrow();
            let href = a.get("href").ok_or(ScrapeError::CssNotFound {
                url: self.url,
                selector: self.chapter_css_selector,
            });

            urls.push(href?.to_string());
        }

        // convert descending list to ascending
        if self.chapter_sort == host::Sorting::Descending {
            &mut urls.reverse();
        }

        // store urls
        self.chapter_urls = Some(urls);

        Ok(())
    }

    fn get_chapter(&self, chapter_number: u32) -> Result<Vec<Vec<u8>>, ScrapeError> {
        // check if the table of contents were scanned
        let chapter_urls = match &self.chapter_urls {
            Some(urls) => urls,
            None => {
                return Err(ScrapeError::TocNotScanned);
            }
        };

        for url in chapter_urls {
            // request chapter html
            let document = kuchiki::parse_html()
                .one(web_util::get_html(url).unwrap().as_string().unwrap())
                .select(self.image_css_selector)
                .unwrap();

            for css_match in document {
                let node = css_match.as_node();
                let img = node.as_element().unwrap().attributes.borrow();
                let src = img.get("src").ok_or(ScrapeError::CssNotFound {
                    url,
                    selector: self.image_css_selector,
                })?;

                web_util::get_html(src).unwrap().as_bytes().unwrap();

                println!("-->{}", src);
            }
        }

        Ok(vec![vec![0]])
    }
}
