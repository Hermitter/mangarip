use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ScrapeError<'a> {
    UnreachableHost { url: &'a str },
    UnsupportedHost { url: &'a str },
    UnreadableHtml { url: &'a str },
    CssNotFound { url: &'a str, selector: &'a str },
    Non200Status { url: &'a str, code: u16 },
}

impl<'a> Error for ScrapeError<'a> {}

impl<'a> fmt::Display for ScrapeError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScrapeError::UnreachableHost { ref url } => {
                write!(f, "Unable to connect to host: {}", url)
            }
            ScrapeError::UnreadableHtml { ref url } => {
                write!(f, "Issue parsing HTML from: {}", url)
            }
            ScrapeError::UnsupportedHost { ref url } => {
                write!(f, "Cannot scrape unsupported host: {} ", url)
            }
            ScrapeError::CssNotFound { ref url, selector } => {
                write!(f, "Selector \"{}\" found nothing from: {}", selector, url)
            }
            ScrapeError::Non200Status { ref url, code } => {
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
