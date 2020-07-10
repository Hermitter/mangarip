use crate::lib::{web::Request, Error};

/// Represents a single page(image) of a chapter.
#[derive(Debug)]
pub struct Page {
    /// direct URL to an image.
    pub url: String,
    /// Bytes that make up an entire image.
    pub content: Vec<u8>,
}

impl Page {
    /// Populate `Page.content` with image found in `Page.url`
    pub async fn download(&self) -> Result<Vec<u8>, Error> {
        Request::new()
            .attempts(10)
            .delay(3)
            .fetch_as_bytes(&self.url)
            .await
    }
}
