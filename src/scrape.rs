use crate::{error::Error, host, web_util};
use futures::future::try_join_all;
use kuchiki;
use kuchiki::traits::*;

/// All the information needed to support a new website
#[derive(Debug)]
pub struct Scraper<'a> {
    /// The url to the table of contents
    // This page contains a link to each chapter in the manga.
    pub url: &'a str,

    /// A list of urls to each chapter in the manga
    pub chapter_urls: Vec<String>,

    /// Describes how the table of contents is sorted.
    /// This will tell us if it's by newest or oldest first.
    pub chapter_sort: host::Sorting,

    /// A selector to each chapter url in the table of contents
    pub chapter_css_selector: &'a str,

    /// A regular expression for parsing image URLs from chapters.
    ///
    /// A bytes-oriented regex is used in order to defer UTF-8 verification to
    /// the image's URL only. This saves verifying the entire document as UTF-8.
    pub image_regex: regex::bytes::Regex,
}

impl<'a> Scraper<'a> {
    /// Check if host is supported and return a Scraper that's configured for target host.
    /// This will also scrape the table of contents to save the URL for each chapter.
    pub async fn from(url: &str) -> Result<Scraper<'_>, Error> {
        let mut scraper = host::find(url)?;

        let mut urls = vec![];

        // download table of contents HTML
        let document = kuchiki::parse_html()
            .one(web_util::get_html(scraper.url).await?.as_string().await?)
            .select(scraper.chapter_css_selector);

        // check if HTML can be parsed
        if !document.is_ok() {
            return Err(Error::UnreadableHtml {
                url: scraper.url.to_owned(),
            });
        }

        // store link for each chapter
        for css_match in document? {
            let node = css_match.as_node();
            let a = node.as_element().unwrap().attributes.borrow();
            let href = a.get("href").ok_or(Error::CssNotFound {
                url: scraper.url.to_owned(),
                selector: scraper.chapter_css_selector.to_owned(),
            });

            urls.push(href?.to_owned());
        }

        // convert descending list to ascending
        if scraper.chapter_sort == host::Sorting::Descending {
            &mut urls.reverse();
        }

        scraper.chapter_urls = urls;
        Ok(scraper)
    }

    /// Request each image from a chapter.
    pub async fn get_chapter(&self, chapter_number: u32) -> Result<Vec<Vec<u8>>, Error> {
        let chapter_url = &self.chapter_urls[chapter_number as usize];

        let html = web_util::get_html(chapter_url).await?.as_bytes().await?;

        // find each image URL in HTML
        let mut image_urls = vec![];
        for captures in self.image_regex.captures_iter(&html) {
            // `capture.iter()` returns an iterator where the first element is
            // the entire match. And its `Item` type is an `Option`, so we can
            // ignore `None` cases by flattening the iterator.
            for capture in captures.iter().skip(1).flatten() {
                match std::str::from_utf8(capture.as_bytes()) {
                    Ok(url) => image_urls.push(url),
                    Err(_) => {
                        return Err(Error::InvalidUtf8 {
                            url: self.url.to_owned(),
                        })
                    }
                }
            }
        }

        // async image source download
        async fn get_image(url: &str) -> Result<Vec<u8>, Error> {
            web_util::get_html(&url).await?.as_bytes().await
        }

        // wait for all images to download
        try_join_all(image_urls.into_iter().map(|x| get_image(x))).await
    }

    /// Request a specific range of chapters.
    pub async fn get_chapters(&self, start: u32, end: u32) -> Result<Vec<Vec<Vec<u8>>>, Error> {
        Ok(vec![vec![vec![0]]])
    }
}
