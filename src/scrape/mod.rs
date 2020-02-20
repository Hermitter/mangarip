#[derive(PartialEq, Debug)]
pub enum Sorting {
    Ascending,
    Descending,
}

pub enum Selector {
    Regex(String),
    Css(String),
}

#[derive(Debug)]
pub struct Request {
    /// Table of Contents
    toc_url: String,
}

impl Request {
    fn new(url: &str) {}
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
