use crate::error::Error;
use async_std::task::sleep;
use std::time::Duration;
use surf;

/// Handles downloading HTML or media from a URL. By default, it will download the HTML string of a web page.
#[derive(Debug)]
pub struct Fetch {
    attempts: u16,
    delay: u64,
}

impl<'a> Fetch {
    /// Return an instance of Request with default values.
    pub fn new() -> Fetch {
        Fetch {
            attempts: 1,
            delay: 1,
        }
    }

    /// Set the number of attempts for a request.
    pub fn attempts(mut self, number: u16) -> Fetch {
        self.attempts = number;
        self
    }

    /// Set the delay of seconds between each attempt for a request.
    pub fn delay(mut self, number: u64) -> Fetch {
        self.delay = number;
        self
    }

    /// Fetch the content of a given URL.
    async fn fetch(mut self, url: &str) -> Result<surf::Response, Error> {
        let mut non_200_status = 0;

        while self.attempts != 0 {
            if let Ok(response) = surf::get(url).await {
                // a 200 status code means the response is ok
                if response.status() == 200 {
                    return Ok(response);
                }

                // save the last failed status code
                self.attempts -= 1;
                if self.attempts == 0 {
                    non_200_status = response.status() as u16;
                }

                sleep(Duration::from_secs(self.delay)).await;
            }
        }

        Err(Error::Non200Status {
            url: url.to_owned(),
            code: non_200_status,
        })
    }

    /// Request and return the HTML string of a URL.
    pub async fn request_html(self, url: &str) -> Result<String, Error> {
        let mut response = self.fetch(url).await?;
        Ok(response.body_string().await?)
    }

    /// Request and return the Bytes of a URL. This is used for media (.png, .jpg, .mp4, etc.).
    pub async fn request_bytes(self, url: &str) -> Result<Vec<u8>, Error> {
        let mut response = self.fetch(url).await?;
        Ok(response.body_bytes().await?)
    }
}
