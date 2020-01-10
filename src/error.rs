use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ScrapeError {
    UnreachableHost(String),
    UnsupportedHost(String),
    UnreadableHtml(String),
    BadCssSelector(String),
    Non200Status(String, i32),
}

impl Error for ScrapeError {}

impl fmt::Display for ScrapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScrapeError::UnsupportedHost(ref url) => {
                write!(f, "Cannot scrape unsupported URL: {} ", url)
            }
            ScrapeError::Non200Status(ref url, code) => {
                write!(f, "{} status code returned from {}", code, url)
            }
            _ => write!(f, "TODO"),
        }
    }
}

// Result<Select<Elements<Descendants>>
// impl From<std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>>
//     for Error
// {
//     fn from(e: <std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>>) -> Self {
//         match e {
//             <std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>> => ScrapeError::UnreadableHtml,
//             _ => Error::InternalServerError,
//         }
//     }
// }
