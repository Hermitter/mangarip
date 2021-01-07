use std::{error::Error as StdError, fmt};
use surf;

#[derive(Debug)]
pub enum Error {
    /// Some unspecified error.
    Any(Box<dyn StdError + Send + Sync + 'static>),
    /// Unable to connect to host.
    UnreachableHost { url: String },
    /// Host is not on supported list.
    UnsupportedHost { url: String },
    /// Could not parse HTML.
    UnreadableHtml { url: String },
    /// CSS selector returned nothing.
    CssNotFound { url: String, selector: String },
    /// Server did not respond with OK 200 status.
    Non200Status { url: String, code: u16 },
    /// The table of contents were not read before scraping.
    TocNotScanned,
    /// Error is not known
    UnknownError,
    /// Encountered invalid UTF-8 in an HTML document.
    InvalidUtf8 { url: String },
    /// Chapter does not exist.
    InvalidChapter { index: u32 },
    /// Surf crate error
    SurfError { err: surf::Error },
}

impl<'a> fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnreachableHost { ref url } => write!(f, "Unable to connect to host: {}", url),
            Error::InvalidChapter { ref index } => {
                write!(f, "Are you sure chapter {} exists?", index)
            }
            Error::UnreadableHtml { ref url } => write!(f, "Issue parsing HTML from: {}", url),
            Error::UnsupportedHost { ref url } => {
                write!(f, "Cannot scrape unsupported host: {} ", url)
            }
            Error::UnknownError => write!(f, "Unable to determine issue"),
            Error::CssNotFound { ref url, selector } => {
                write!(f, "Selector \"{}\" found nothing from: {}", selector, url)
            }
            Error::TocNotScanned => {
                write!(f, "Table of contents were not scanned for chapter URLs")
            }

            Error::Non200Status { ref url, code } => {
                write!(f, "{} status code returned from {}", code, url)
            }
            Error::SurfError { ref err } => {
                write!(f, "{}", err.to_string())
            }
            _ => write!(f, "TODO"),
        }
    }
}

impl<'a> From<()> for Error {
    fn from(_err: ()) -> Self {
        Error::UnknownError
    }
}

impl<'a> From<surf::Error> for Error {
    fn from(err: surf::Error) -> Self {
        Error::SurfError { err }
    }
}
