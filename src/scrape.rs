use crate::{error::Error, host, web_util};
use futures::channel::mpsc;
use futures::future::try_join_all;
use kuchiki;
use kuchiki::traits::*;
use std::boxed::Box;
use std::time::Duration;
use tokio;

/// All the information needed to scrape a website.
pub struct Scraper<'a> {
    /// The url to the table of contents
    // This page contains a link to each chapter in the manga.
    pub url: &'a str,

    /// A list of urls to each chapter in the manga
    pub chapter_urls: Vec<String>,

    /// Describes if chapters are listed from newest to oldest or vice versa.
    pub chapter_sort: host::Sorting,

    /// A selector to each chapter url in the table of contents
    pub chapter_css_selector: &'a str,

    /// A regular expression for parsing image URLs from chapters.
    ///
    /// A bytes-oriented regex is used in order to defer UTF-8 verification to
    /// the image's URL only. This saves verifying the entire document as UTF-8.
    pub image_regex: regex::bytes::Regex,

    /// A closure that's given the number of the last downloaded chapter.
    /// Only used by functions that involves multiple chapter downloads.
    pub chapter_closure: Option<Box<dyn Fn(Result<Vec<Vec<u8>>, Error>, u32) + Send + Sync + 'a>>,
}

impl<'a> Scraper<'a> {
    /// Check if host is supported and return a Scraper that's configured for target host.
    /// This will also scrape the table of contents to save the URL for each chapter.
    pub async fn from(url: &'a str) -> Result<Scraper<'a>, Error> {
        let mut scraper = host::find(url)?;
        scraper.chapter_closure = None;

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

    /// Request a specified range of chapters. Each chapter will have all images inside.
    pub async fn get_chapters(&self, start: u32, end: u32) -> Result<(), Error> {
        // let mut book = Vec::new();
        let mut handlers = Vec::new();
        let (tx, mut rx) = mpsc::channel(1_024);

        for i in start..end {
            let image_regex = self.image_regex.clone(); // waste to clone each time
            let url = self.chapter_urls[i as usize].clone();
            let mut tx2 = tx.clone();

            let x = tokio::time::delay_for(Duration::new(5, 0));

            handlers.push(tokio::spawn(async move {
                let chapter = get_chapter(url, image_regex)
                    .await
                    .expect("could not get chapter");
                println!("FINISHED NEW SCAN: {}", i);
                tx2.try_send((chapter, i)).expect("Failed to send chapter");
            }));
        }

        for handle in handlers {
            handle.await.unwrap();

            if let Some(function) = &self.chapter_closure {
                if let Ok(chapter) = rx.try_next() {
                    let (chapter, i) = chapter
                        .ok_or(Error::UnknownError)
                        .expect("Failed to receive chapter");
                // function(chapter, i);
                } else {
                    println!("COULD NOT FIND NEW MESSAGES");
                }
            }
        }

        Ok(())
    }

    /// Sets the chapter_closure property for Scraper.
    pub fn on_chapter_finish<F>(&mut self, f: F)
    where
        F: 'a,
        F: Send,
        F: Sync,
        F: Fn(Result<Vec<Vec<u8>>, Error>, u32),
    {
        self.chapter_closure = Some(Box::new(f));
    }
}

/////////////////////////////////////////////////
/// TESTING
/// Request each image from a chapter.
pub async fn get_chapter(
    url: String,
    image_regex: regex::bytes::Regex,
) -> Result<Vec<Vec<u8>>, Error> {
    let html = web_util::get_html(&url).await?.as_bytes().await?;

    // find each image URL in HTML
    let mut image_urls = vec![];
    for captures in image_regex.captures_iter(&html) {
        // `capture.iter()` returns an iterator where the first element is
        // the entire match. And its `Item` type is an `Option`, so we can
        // ignore `None` cases by flattening the iterator.
        for capture in captures.iter().skip(1).flatten() {
            match std::str::from_utf8(capture.as_bytes()) {
                Ok(url) => image_urls.push(url),
                Err(_) => {
                    return Err(Error::InvalidUtf8 {
                        url: url.to_owned(),
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
