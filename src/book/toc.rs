use super::Host;
use crate::url::Request;
use crate::Error;
use crate::{Selector, Sorting};
use kuchiki;
use kuchiki::traits::*;
use regex::Regex;

/// Information on the table of contents for the website. This page should contain a link to each chapter available.
#[derive(Debug)]
pub struct TableOfContents {
    /// URL to the table of contents.
    pub url: String,
    /// Describes how the chapters are sorted.
    pub sorting: Sorting,
    /// Determines what selector type is used.
    pub selector: Selector,
    /// Links to each chapter in a manga.
    pub chapter_urls: Vec<String>,
}

impl TableOfContents {
    /// Return a new instance of a TableOfContents for a book.
    pub fn new(url: &str, host: &Host) -> TableOfContents {
        TableOfContents {
            url: url.to_owned(),
            sorting: host.toc_sorting,
            selector: host.chapter_selector.clone(),
            chapter_urls: Vec::new(),
        }
    }

    /// Populate chapter_urls with a url to each chapter.
    pub async fn scan(&mut self) -> Result<(), Error> {
        // fetch html to scan
        let html = Request::new()
            .attempts(3)
            .fetch_as_string(&self.url)
            .await?;

        // populate chapter URLs with each chapter found
        match &self.selector {
            Selector::Css(pattern) => {
                let document = kuchiki::parse_html().one(html).select(&pattern);

                for css_match in document? {
                    let node = css_match.as_node();
                    let a = node.as_element().unwrap().attributes.borrow();
                    let href = a.get("href").ok_or(Error::CssNotFound {
                        url: self.url.clone(),
                        selector: pattern.clone(),
                    });

                    self.chapter_urls.push(href?.to_owned());
                }
            }
            Selector::Regex(_) => {
                panic!("Regex Selector is not yet implemented for Table of Contents!")
                // let regex = Regex::new(pattern).unwrap();
                // for captures in regex.captures_iter(&html) {
                //     // `capture.iter()` returns an iterator where the first element is
                //     // the entire match. And its `Item` type is an `Option`, so we can
                //     // ignore `None` cases by flattening the iterator.
                //     for capture in captures.iter().skip(1).flatten() {
                //         match std::str::from_utf8(capture.as_bytes()) {
                //             Ok(url) => self.chapter_urls.push(url.to_owned()),
                //             Err(_) => {
                //                 return Err(Error::InvalidUtf8 {
                //                     url: self.url.to_owned(),
                //                 })
                //             }
                //         }
                //     }
                // }
            }
        }

        if Sorting::Descending == self.sorting {
            self.chapter_urls.reverse();
        }

        Ok(())
    }
}
