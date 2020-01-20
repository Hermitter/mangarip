use crate::error::Error;
use surf;

// consumable that returns Bytes or a String of a web page
#[derive(Debug)]
pub struct Html<'a> {
    response: surf::Response,
    url: &'a str,
}

impl<'a> Html<'a> {
    pub async fn as_string(mut self) -> Result<String, Error> {
        if let Ok(html) = self.response.body_string().await {
            Ok(html)
        } else {
            Err(Error::UnreadableHtml {
                url: self.url.to_owned(),
            })
        }
    }

    pub async fn as_bytes(mut self) -> Result<Vec<u8>, Error> {
        if let Ok(bytes) = self.response.body_bytes().await {
            Ok(bytes)
        } else {
            Err(Error::UnreadableHtml {
                url: self.url.to_owned(),
            })
        }
    }
}

// Return an HTML string of the table of contents page
pub async fn get_html(url: &str) -> Result<Html<'_>, Error> {
    // request web page
    if let Ok(response) = surf::get(url).await {
        // ensure 200 OK response
        if response.status() != 200 {
            return Err(Error::Non200Status {
                url: url.to_owned(),
                code: response.status().as_u16(),
            });
        }

        // response can be consumed to return html as bytes or a string
        return Ok(Html { response, url });
    }

    Err(Error::UnreachableHost {
        url: url.to_owned(),
    })
}
