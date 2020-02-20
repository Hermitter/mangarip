use crate::{Selector, Sorting};
mod toc;
use toc::TableOfContents;

/// Information needed to support a new manga website.
#[derive(Debug)]
pub struct Host {
    /// URL to host website.
    pub url: String,
    /// Describes how the chapter list is sorted.
    pub toc_sorting: Sorting,
    /// Selector for chapter url in the table of contents.
    pub toc_selector: Selector,
    /// Selector for each image url in a chapter.
    pub page_selector: Selector,
}

#[derive(Debug)]
pub struct Chapter {}

#[derive(Debug)]
pub struct Book {
    /// The website that's hosting the manga.
    host: Host,

    /// Table of contents for a manga.
    toc: TableOfContents,
}

// /// All the information needed to support a new website
// #[derive(Debug)]
// pub struct Scraper {
//     /// url to the table of contents
//     pub url: String,

//     /// urls to each chapter in the manga
//     pub chapter_urls: Vec<String>,

//     /// how the table of contents is sorted
//     pub chapter_sort: host::Sorting,

//     /// selector to each chapter url in the table of contents
//     pub chapter_css_selector: &'a str,

//     /// selector to each image url in a chapter
//     pub image_css_selector: &'a str,
// }
