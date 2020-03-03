use super::chapter::Chapter;
use crate::{url::Request, Error, Selector, Sorting};
use kuchiki;
use kuchiki::traits::*;
use regex::Regex;
use std::cell::RefCell;

/// Information needed to support a new manga website.
#[derive(Debug)]
pub struct Host {
    /// Describes how the chapter list is sorted in the table of contents.
    pub toc_sorting: Sorting,
    /// Selector for each chapter url in the table of contents.
    pub chapter_selector: Selector,
    /// Selector for each image url in a chapter.
    pub page_selector: Selector,
    /// String to append at the end of a chapter's URL.
    pub chapter_url_append: String,
}

impl<'a> Host {
    /// Populate chapter_urls with a url to each chapter.
    pub async fn scan(&self, url: &str) -> Result<Vec<RefCell<Chapter>>, Error> {
        let mut chapters = Vec::new();

        // fetch html to scan
        let html = Request::new().attempts(3).fetch_as_string(&url).await?;

        // populate chapter URLs with each chapter found
        match &self.chapter_selector {
            Selector::Css(pattern) => {
                let document = kuchiki::parse_html().one(html).select(&pattern);

                for css_match in document? {
                    let node = css_match.as_node();
                    let a = node.as_element().unwrap().attributes.borrow();
                    let href = a.get("href").ok_or(Error::CssNotFound {
                        url: url.to_owned(),
                        selector: pattern.clone(),
                    });

                    chapters.push(RefCell::new(Chapter {
                        url: href?.to_owned(),
                        pages: Vec::new(),
                    }));
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
                //                     url: url.to_owned(),
                //                 })
                //             }
                //         }
                //     }
                // }
            }
        }

        if Sorting::Descending == self.toc_sorting {
            chapters.reverse();
        }

        Ok(chapters)
    }
}
