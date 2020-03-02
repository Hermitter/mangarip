mod error;
pub use error::Error;
pub mod book;
use regex;
pub mod url;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Sorting {
    Ascending,
    Descending,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Selector {
    Regex(String),
    Css(String),
}
