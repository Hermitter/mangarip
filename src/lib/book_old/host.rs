use super::chapter::Chapter;
use crate::lib::{web::Request, Error, Selector, Sorting};
use kuchiki;
use kuchiki::traits::*;
use regex::Regex;

/// Information needed to support a new manga website.
#[derive(Debug)]
pub struct Host {
    /// Describes how the chapter list is sorted in the table of contents.
    pub toc_sorting: Sorting,
    /// Selector for each chapter url in the table of contents.
    pub chapter_selector: Selector,
    /// Selector for each image url in a chapter.
    pub image_selector: Selector,
    /// String to append at the end of each chapter's URL. This meant for sites that
    /// require a specific path to load each image at once.
    pub chapter_url_append: String,
}

impl<'a> Host {
    /// Return the url of each chapter in the table of contents
    pub async fn scan(&self, url: &str) -> Result<Vec<Chapter>, Error> {
        let mut chapters = Vec::new();
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
                    })?;

                    chapters.push(Chapter {
                        url: href.to_owned(),
                        pages: Vec::new(),
                    });
                }
            }
            Selector::Regex(_pattern) => {
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
