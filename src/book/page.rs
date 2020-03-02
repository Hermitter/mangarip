use crate::{url::Request, Error};

/// Represents a single page(image) of a chapter.
#[derive(Debug)]
pub struct Page {
    /// direct URL to an image.
    pub url: String,
    /// Bytes that make up an entire image.
    pub content: Vec<u8>,
}

impl Page {
    pub async fn download(&mut self) -> Result<(), Error> {
        self.content = Request::new()
            .attempts(10)
            .delay(3)
            .fetch_as_bytes(&self.url)
            .await?;

        Ok(())
    }
}
