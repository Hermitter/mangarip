use crate::error::ScrapeError;
use async_std::task;
use std::io;
use surf;

// consumable that returns Bytes or a String of a web page
pub struct Html<'a> {
    response: surf::Response,
    url: &'a str,
}

impl<'a> Html<'a> {
    pub fn as_string(mut self) -> Result<String, ScrapeError<'a>> {
        task::block_on(async {
            if let Ok(html) = self.response.body_string().await {
                Ok(html)
            } else {
                Err(ScrapeError::UnreadableHtml { url: self.url })
            }
        })
    }

    pub fn as_bytes(mut self) -> Result<Vec<u8>, ScrapeError<'a>> {
        task::block_on(async {
            if let Ok(html) = self.response.body_bytes().await {
                Ok(html)
            } else {
                Err(ScrapeError::UnreadableHtml { url: self.url })
            }
        })
    }
}

// Return an HTML string of the table of contents page
pub fn get_html(url: &str) -> Result<Html, ScrapeError> {
    task::block_on(async {
        // request web page
        if let Ok(response) = surf::get(url).await {
            // ensure 200 OK response
            if response.status() != 200 {
                return Err(ScrapeError::Non200Status {
                    url,
                    code: response.status().as_u16(),
                });
            }

            // response can be consumed to return html as bytes or a string
            return Ok(Html { response, url });
        }

        Err(ScrapeError::UnreachableHost { url })
    })
}
