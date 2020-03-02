use crate::error::Error;
use async_std::task::sleep;
use std::time::Duration;
use surf;

/// Handles downloading HTML or media from a URL.
pub struct Request {
    pub attempts: u16,
    pub delay: u16,
}

impl<'a> Request {
    /// Return an instance of Request with default values.
    pub fn new() -> Request {
        Request {
            attempts: 1,
            delay: 1,
        }
    }

    /// Set the number of attempts for a request.
    pub fn attempts(&'a mut self, number: u16) -> &'a mut Request {
        self.attempts = number;
        self
    }

    /// Set the delay of seconds between each attempt for a request.
    pub fn delay(&'a mut self, number: u16) -> &'a mut Request {
        self.delay = number;
        self
    }

    /// Initiate request and return an HTML string.
    pub async fn fetch_as_string(&self, url: &str) -> Result<String, Error> {
        Response::fetch(url, self.attempts).await?.as_string().await
    }

    /// Initiate request and return a byte array. Used for media content.
    pub async fn fetch_as_bytes(&self, url: &str) -> Result<Vec<u8>, Error> {
        Response::fetch(url, self.attempts).await?.as_bytes().await
    }
}

/// Results of a webpage that can be turned into a Byte or a String representation.
#[derive(Debug)]
struct Response {
    pub response: surf::Response,
    pub url: String,
}

impl Response {
    /// Request a webpage's contentS
    async fn fetch(url: &str, attempts: u16) -> Result<Response, Error> {
        let mut attempts = attempts;
        let mut non_200_code = 0;

        while attempts != 0 {
            // request url
            if let Ok(response) = surf::get(url).await {
                // finish if ok response
                if response.status() == 200 {
                    return Ok(Response {
                        response,
                        url: url.to_owned(),
                    });
                }

                // if not okay response
                attempts -= 1;
                if attempts == 0 {
                    non_200_code = response.status().as_u16();
                }

                // wait before trying again
                sleep(Duration::from_secs(1)).await;
            }
        }

        Err(Error::Non200Status {
            url: url.to_owned(),
            code: non_200_code,
        })
    }

    async fn as_string(mut self) -> Result<String, Error> {
        if let Ok(html) = self.response.body_string().await {
            Ok(html)
        } else {
            Err(Error::UnreadableHtml {
                url: self.url.to_owned(),
            })
        }
    }

    async fn as_bytes(mut self) -> Result<Vec<u8>, Error> {
        if let Ok(bytes) = self.response.body_bytes().await {
            Ok(bytes)
        } else {
            Err(Error::UnreadableHtml {
                url: self.url.to_owned(),
            })
        }
    }
}
