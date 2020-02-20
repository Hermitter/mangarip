use crate::Error;
use crate::{Selector, Sorting};

/// Information on the table of contents for the website. This page should contain a link to each chapter available.
#[derive(Debug)]
pub struct TableOfContents {
    /// URL to the table of contents.
    pub url: String,
    /// Describes how the chapters are sorted.
    pub sorting: Sorting,
    /// Determines what selector function is used.
    pub selector: Selector,
    /// Links to each chapter in a manga.
    pub chapter_urls: Vec<String>,
}

// impl TableOfContents {
//     /// Check if host is supported and return a Scrape with host's configuration.
//     pub async fn new(url: &str) -> Result<(), Error> {
//         let mut scraper = host::find(url)?;

//         let mut urls = vec![];

//         // download table of contents HTML
//         let document = kuchiki::parse_html()
//             .one(web_util::get_html(scraper.url).await?.as_string().await?)
//             .select(scraper.chapter_css_selector);

//         // check if HTML can be parsed
//         if !document.is_ok() {
//             return Err(Error::UnreadableHtml {
//                 url: scraper.url.to_owned(),
//             });
//         }

//         // store link for each chapter
//         for css_match in document? {
//             let node = css_match.as_node();
//             let a = node.as_element().unwrap().attributes.borrow();
//             let href = a.get("href").ok_or(Error::CssNotFound {
//                 url: scraper.url.to_owned(),
//                 selector: scraper.chapter_css_selector.to_owned(),
//             });

//             urls.push(href?.to_owned());
//         }

//         // convert descending list to ascending
//         if scraper.chapter_sort == host::Sorting::Descending {
//             &mut urls.reverse();
//         }

//         scraper.chapter_urls = urls;
//         Ok(scraper)
//     }
// }
