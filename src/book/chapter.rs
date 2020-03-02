use super::page::Page;
use crate::url::Request;
use crate::{Error, Selector};
use std::cell::RefCell;

/// Contains every page(image) of a specific chapter.
#[derive(Debug)]
pub struct Chapter {
    /// URL to a chapter.
    pub url: String,
    /// Each image from a chapter.
    pub pages: Vec<RefCell<Page>>,
}

impl Chapter {
    /// Populates each page with a url to it.
    pub async fn scan(&mut self, selector: &Selector) -> Result<(), Error> {
        if !self.pages.is_empty() {
            return Ok(());
        }

        let request = Request {
            attempts: 5,
            delay: 1,
        };

        match selector {
            Selector::Css(pattern) => {
                panic!("CSS Selector is not yet implemented for Chapter pages");
            }
            Selector::Regex(pattern) => {
                let pattern = regex::bytes::Regex::new(pattern).unwrap();
                let html = request.fetch_as_bytes(&self.url).await?;

                for captures in pattern.captures_iter(&html) {
                    // `capture.iter()` returns an iterator where the first element is
                    // the entire match. And its `Item` type is an `Option`, so we can
                    // ignore `None` cases by flattening the iterator.
                    for capture in captures.iter().skip(1).flatten() {
                        match std::str::from_utf8(capture.as_bytes()) {
                            Ok(url) => self.pages.push(RefCell::new(Page {
                                url: url.to_owned(),
                                content: Vec::new(),
                            })),
                            Err(_) => {
                                return Err(Error::InvalidUtf8 {
                                    url: self.url.to_owned(),
                                })
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}