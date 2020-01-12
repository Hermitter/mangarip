use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ScrapeError<'a> {
    UnreachableHost { url: &'a str },
    UnsupportedHost { url: &'a str },
    UnreadableHtml { url: &'a str },
    CssNotFound { url: &'a str, selector: &'a str },
    Non200Status { url: &'a str, code: u16 },
    UnknownError,
    TocNotScanned,
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
            ScrapeError::TocNotScanned => {
                write!(f, "Table of contents were not scanned for chapter URLs")
            }

            ScrapeError::UnknownError => write!(f, "Unable to determine issue"),

            ScrapeError::Non200Status { ref url, code } => {
                write!(f, "{} status code returned from {}", code, url)
            } // _ => write!(f, "TODO"),
        }
    }
}

impl<'a> From<()> for ScrapeError<'a> {
    fn from(_err: ()) -> Self {
        ScrapeError::UnknownError
    }
}
