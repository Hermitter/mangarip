use crate::{error::ScrapeError, host, scrape::Fetch, web_util};
use kuchiki;
use kuchiki::traits::*;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// All the information needed to support a new website
#[derive(Debug)]
pub struct Scraper<'a> {
    /// url to the table of contents
    pub url: &'a str,

    /// urls to each chapter in the manga
    pub chapter_urls: Vec<String>,

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
        let mut scraper = host::find(url)?;

        let mut urls = vec![];

        // download table of contents HTML
        let document = kuchiki::parse_html()
            .one(web_util::get_html(scraper.url)?.as_string()?)
            .select(scraper.chapter_css_selector);

        // check if HTML can be parsed
        if !document.is_ok() {
            return Err(ScrapeError::UnreadableHtml { url: scraper.url });
        }

        // store link for each chapter
        for css_match in document? {
            let node = css_match.as_node();
            let a = node.as_element().unwrap().attributes.borrow();
            let href = a.get("href").ok_or(ScrapeError::CssNotFound {
                url: scraper.url,
                selector: scraper.chapter_css_selector,
            });

            urls.push(href?.to_string());
        }

        // convert descending list to ascending
        if scraper.chapter_sort == host::Sorting::Descending {
            &mut urls.reverse();
        }

        scraper.chapter_urls = urls;
        Ok(scraper)
    }
}

impl<'a> Fetch for Scraper<'a> {
    fn get_chapter(&self, chapter_number: u32) -> Result<Vec<Vec<u8>>, ScrapeError> {
        Ok(download_chapter(
            &self.chapter_urls[chapter_number as usize],
            self.image_css_selector,
        ))
    }
}

fn download_chapter(url: &str, css_selector: &str) -> Vec<Vec<u8>> {
    // fetch webpage HTML
    let document = kuchiki::parse_html()
        .one(web_util::get_html(url).unwrap().as_string().unwrap())
        .select(css_selector)
        .unwrap();

    // parse HTML for each image URL
    let mut image_urls = vec![];

    for css_match in document {
        let node = css_match.as_node();
        let img = node.as_element().unwrap().attributes.borrow();
        let src = img
            .get("src")
            .ok_or(ScrapeError::CssNotFound {
                url,
                selector: css_selector,
            })
            .unwrap();
        image_urls.push(src.to_string());
    }

    // fetch each image
    let mut chapter = vec![];

    for url in image_urls {
        chapter.push(web_util::get_html(&url).unwrap().as_bytes().unwrap());
    }

    chapter
}

// multi threading example
/*
    // fetch each image
    let images = image_urls.len();
    let mut chapter = vec![];
    let (tx, rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) = mpsc::channel();

    // multi threaded image scraping
    let mut handlers = Vec::new();

    while !image_urls.is_empty() {
        let thread_tx = tx.clone();
        let url = image_urls.pop().unwrap();

        handlers.push(thread::spawn(move || {
            thread_tx
                .send(web_util::get_html(&url).unwrap().as_bytes().unwrap())
                .unwrap();
        }));
    }

    println!("collected images");

    // collect data
    for _ in 0..images {
        chapter.push(rx.recv().unwrap());
    }

    println!("collected data");

    // start threads
    for handle in handlers {
        handle.join().unwrap();
        println!("collected threads");
    }

    chapter
*/
