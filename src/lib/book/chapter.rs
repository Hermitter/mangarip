use crate::lib::web::Request;
use crate::lib::Error;
use crate::lib::{Selector, Sorting};
/// Representation of a manga chapter.
#[derive(Debug)]
pub struct Chapter {
    /// URL to a chapter.
    pub url: String,
    /// URL to each image found.
    pub image_urls: Vec<String>,
}

impl Chapter {
    /// Populates each page with a url to it.
    pub async fn get_image_urls(&mut self, selector: &Selector) -> Result<(), Error> {
        if !self.image_urls.is_empty() {
            return Ok(());
        }

        match selector {
            Selector::Css(pattern) => {
                panic!("CSS Selector is not yet implemented for Chapter pages");
            }
            Selector::Regex(pattern) => {
                let pattern = regex::bytes::Regex::new(pattern).unwrap();
                let html = Request::new()
                    .attempts(5)
                    .delay(1)
                    .fetch_as_bytes(&self.url)
                    .await?;

                for captures in pattern.captures_iter(&html) {
                    // `capture.iter()` returns an iterator where the first element is
                    // the entire match. And its `Item` type is an `Option`, so we can
                    // ignore `None` cases by flattening the iterator.
                    for capture in captures.iter().skip(1).flatten() {
                        match std::str::from_utf8(capture.as_bytes()) {
                            Ok(url) => self.image_urls.push(url.to_owned()),
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
