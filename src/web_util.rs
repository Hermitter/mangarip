use crate::error::ScrapeError;
use async_std::task;
use std::io;
use surf;

// Return an HTML string of the table of contents page
pub fn get_html(url: &str) -> Result<String, ScrapeError> {
    task::block_on(async {
        if let Ok(mut reader) = surf::get(url).await {
            if reader.status() != 200 {
                return Err(ScrapeError::Non200Status {
                    url,
                    code: reader.status().as_u16(),
                });
            }
            if let Ok(html) = reader.body_string().await {
                return Ok(html);
            }
        }
        return Err(ScrapeError::UnreadableHtml { url });
    })
}

// Return an HTML string of the table of contents
pub fn get_bytes(url: &str) -> Result<io::Result<Vec<u8>>, ScrapeError> {
    Ok(task::block_on(async {
        let mut reader = surf::get(url).await.expect("Failed to fetch URL");

        if reader.status() != 200 {
            panic!("Website response was not OK");
        }

        Ok(reader.body_bytes().await.expect("Failed in parsing URL"))
    }))
}
