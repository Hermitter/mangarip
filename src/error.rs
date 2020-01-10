use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ScrapeError {
    UnsupportedHost(String),
    UnreadableHTML(String),
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
// impl From<DieselError> for Error {
//     fn from(e: DieselError) -> Self {
//         match e {
//             DieselError::NotFound => Error::NotFound,
//             _ => Error::InternalServerError,
//         }
//     }
// }
