use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error<'a> {
    /// Some unspecified error.
    Any(Box<dyn StdError + Send + Sync + 'static>),
    /// Unable to connect to host.
    UnreachableHost { url: &'a str },
    /// Host is not on supported list.
    UnsupportedHost { url: &'a str },
    /// Could not parse HTML.
    UnreadableHtml { url: &'a str },
    /// CSS selector returned nothing.
    CssNotFound { url: &'a str, selector: &'a str },
    /// Server did not respond with OK 200 status.
    Non200Status { url: &'a str, code: u16 },
    /// Error is not known
    UnknownError,
    /// The table of contents were not read before scraping.
    TocNotScanned,
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnreachableHost { ref url } => write!(f, "Unable to connect to host: {}", url),
            Error::UnreadableHtml { ref url } => write!(f, "Issue parsing HTML from: {}", url),
            Error::UnsupportedHost { ref url } => {
                write!(f, "Cannot scrape unsupported host: {} ", url)
            }
            Error::CssNotFound { ref url, selector } => {
                write!(f, "Selector \"{}\" found nothing from: {}", selector, url)
            }
            Error::TocNotScanned => {
                write!(f, "Table of contents were not scanned for chapter URLs")
            }

            Error::UnknownError => write!(f, "Unable to determine issue"),

            Error::Non200Status { ref url, code } => {
                write!(f, "{} status code returned from {}", code, url)
            }
            _ => write!(f, "TODO"),
        }
    }
}

impl<'a> From<()> for Error<'a> {
    fn from(_err: ()) -> Self {
        Error::UnknownError
    }
}
