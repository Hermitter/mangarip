use super::chapter::Chapter;
use super::fetch::Fetch;
use super::{Selector, Sorting};
use crate::Error;
use kuchiki::traits::*;
use url::Url;

/// Information needed to support a new manga website.
#[derive(Debug)]
pub struct Host<'a> {
    /// Sorting for table of contents.
    pub toc_sorting: Sorting,
    /// Selector for finding chapter urls in the table of contents.
    pub chapter_selector: Selector,
    /// Selector for each image url in a chapter.
    pub image_selector: Selector,
    /// String to append at the end of each chapter's URL. Some sites require
    /// require a specific path to load each image at once.
    pub chapter_url_append: Option<&'a str>,
}

impl<'a> Host<'a> {
    /// Return the url of each chapter in the table of contents
    pub async fn get_chapters(&self, url: &str) -> Result<Vec<Chapter>, Error> {
        let mut chapters = Vec::new();
        let html = Fetch::new().attempts(5).request_html(&url).await?;

        // populate chapter URLs with each chapter found
        match &self.chapter_selector {
            Selector::Css(pattern) => {
                let document = kuchiki::parse_html().one(html).select(&pattern);

                for css_match in document? {
                    let node = css_match.as_node();
                    let a = node.as_element().unwrap().attributes.borrow();
                    let href = a.get("href").ok_or(Error::CssNotFound {
                        url: format!("{}{}", url, &self.chapter_url_append.unwrap_or("")),
                        selector: pattern.clone(),
                    })?;

                    chapters.push(Chapter {
                        url: href.to_owned(),
                        image_urls: Vec::new(),
                    });
                }
            }
            Selector::Regex(_pattern) => {
                // TODO: Add regex support
                panic!("Regex Selector is not yet implemented for Table of Contents!")
            }
        }

        if Sorting::Descending == self.toc_sorting {
            chapters.reverse();
        }

        Ok(chapters)
    }

    /// Returns a [Host](Host) from a valid URL it's already supported.
    pub fn find_host(url: &str) -> Option<Host> {
        match Url::parse(url).ok()?.host_str()? {
            "manganelo.com" => Some(Host {
                toc_sorting: Sorting::Descending,
                chapter_selector: Selector::Css(".row-content-chapter li a".to_owned()),
                image_selector: Selector::Regex(r#"src *= *"([^"]+/\d+\.(?:jpg|png))""#.to_owned()),
                chapter_url_append: None,
            }),
            _ => None,
        }
    }
}
