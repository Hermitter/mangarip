mod error;
pub use error::Error;
pub mod book;
pub mod url;

#[derive(PartialEq, Debug)]
pub enum Sorting {
    Ascending,
    Descending,
}

#[derive(PartialEq, Debug)]
pub enum Selector {
    Regex(String),
    Css(String),
}
