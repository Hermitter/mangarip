use std::{error::Error as StdError, fmt};
use surf;

#[derive(Debug)]
pub enum Error {
    /// Some unspecified error.
    Any(Box<dyn StdError + Send + Sync + 'static>),
    /// CSS selector returned nothing.
    CssNotFound { url: String, selector: String },
    /// Server did not respond with OK 200 status.
    Non200Status { url: String, code: u16 },
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
            Error::InvalidChapter { ref index } => {
                write!(f, "Are you sure chapter {} exists?", index)
            }
            Error::UnknownError => write!(f, "Unable to determine issue"),
            Error::CssNotFound { ref url, selector } => {
                write!(f, "Selector \"{}\" found nothing from: {}", selector, url)
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
