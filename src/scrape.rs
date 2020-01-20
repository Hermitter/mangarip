use crate::{error::Error, host, web_util};
use futures::future::{self, try_join_all};
use kuchiki;
use kuchiki::traits::*;

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
                url: scraper.url.to_string(),
            });
        }

        // store link for each chapter
        for css_match in document? {
            let node = css_match.as_node();
            let a = node.as_element().unwrap().attributes.borrow();
            let href = a.get("href").ok_or(Error::CssNotFound {
                url: scraper.url.to_string(),
                selector: scraper.chapter_css_selector.to_string(),
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

    pub async fn get_chapter(&self, chapter_number: u32) -> Result<Vec<Vec<u8>>, Error> {
        let chapter_url = &self.chapter_urls[chapter_number as usize];
        let img_css_select = &self.image_css_selector;

        // fetch webpage HTML
        let document = kuchiki::parse_html()
            .one(web_util::get_html(chapter_url).await?.as_string().await?)
            .select(img_css_select)?;

        // find each image URL in HTML
        let mut image_urls = vec![];
        for css_match in document {
            let node = css_match.as_node();
            let img = node.as_element().unwrap().attributes.borrow();

            let src = img.get("src").ok_or(Error::CssNotFound {
                url: chapter_url.to_string(),
                selector: img_css_select.to_string(),
            })?;

            image_urls.push(src.to_string());
        }

        // downloads the image source
        async fn get_image(url: String) -> Result<Vec<u8>, Error> {
            web_util::get_html(&url).await?.as_bytes().await
        }

        // wait for all images to download
        try_join_all(image_urls.into_iter().map(|x| get_image(x))).await
    }
}

// Return all images from a chapter.
// fn download_chapter<'a>(url: &'a str, css_selector: &'a str) -> Result<Vec<Vec<u8>>, Error<'a>> {
//     // fetch webpage HTML
//     let document = kuchiki::parse_html()
//         .one(web_util::get_html(url)?.as_string()?)
//         .select(css_selector)?;

//     // parse HTML for each image URL
//     let mut image_urls = vec![];
//     for css_match in document {
//         let node = css_match.as_node();
//         let img = node.as_element().unwrap().attributes.borrow();
//         let src = img.get("src").ok_or(Error::CssNotFound {
//             url,
//             selector: css_selector,
//         })?;
//         image_urls.push(src.to_string());
//     }

//     // request each image
//     // let mut images = Vec::new();

//     // for url in image_urls {
//     //     println!("{}", url);
//     //     handlers.push(tokio::spawn(async move { web_util::get_html(&url) }));
//     // }

//     Ok(vec![vec![0]])
// }
