mod error;
pub use error::Error;
pub mod book;
pub mod web;

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
