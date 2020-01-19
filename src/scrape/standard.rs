use crate::{error::Error, host, scrape::Fetch, web_util};
use kuchiki;
use kuchiki::traits::*;
use tokio;
use tokio::prelude::*;

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
    /// Check if host is supported and return a Scrape with host's configuration.
    pub fn from(url: &str) -> Result<Scraper, Error> {
        let mut scraper = host::find(url)?;

        let mut urls = vec![];

        // download table of contents HTML
        let document = kuchiki::parse_html()
            .one(web_util::get_html(scraper.url)?.as_string()?)
            .select(scraper.chapter_css_selector);

        // check if HTML can be parsed
        if !document.is_ok() {
            return Err(Error::UnreadableHtml { url: scraper.url });
        }

        // store link for each chapter
        for css_match in document? {
            let node = css_match.as_node();
            let a = node.as_element().unwrap().attributes.borrow();
            let href = a.get("href").ok_or(Error::CssNotFound {
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
    fn get_chapter(&self, chapter_number: u32) -> Result<Vec<Vec<u8>>, Error> {
        download_chapter(
            &self.chapter_urls[chapter_number as usize].clone(),
            self.image_css_selector,
        )
    }
}

/// Return all images from a chapter.
async fn download_chapter<'a>(
    url: &'a str,
    css_selector: &'a str,
) -> Result<Vec<Vec<u8>>, Error<'a>> {
    // fetch webpage HTML
    let document = kuchiki::parse_html()
        .one(web_util::get_html(url)?.as_string()?)
        .select(css_selector)?;

    // parse HTML for each image URL
    let mut image_urls = vec![];
    for css_match in document {
        let node = css_match.as_node();
        let img = node.as_element().unwrap().attributes.borrow();
        let src = img.get("src").ok_or(Error::CssNotFound {
            url,
            selector: css_selector,
        })?;
        image_urls.push(src.to_string());
    }

    // request each image
    let mut handlers = Vec::new();
    let mut images = Vec::new();

    for url in image_urls {
        println!("{}", url);
        handlers.push(tokio::spawn(async move { web_util::get_html(&url) }));
    }

    //
    for h in handlers {
        images.push(h.await.unwrap()?.as_bytes()?);
    }

    Ok(images)
}
