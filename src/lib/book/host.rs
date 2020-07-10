use super::super::{web::Request, Error, Selector, Sorting};
use super::chapter::Chapter;
use kuchiki::traits::*;

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
    pub chapter_url_append: Option<String>,
}

impl Host {
    /// Return the url of each chapter in the table of contents
    pub async fn get_chapters(&self, url: &str) -> Result<Vec<Chapter>, Error> {
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
                        image_urls: Vec::new(),
                    });
                }
            }
            Selector::Regex(_pattern) => {
                panic!("Regex Selector is not yet implemented for Table of Contents!")
            }
        }

        if Sorting::Descending == self.toc_sorting {
            chapters.reverse();
        }

        Ok(chapters)
    }
}
