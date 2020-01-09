use async_std::task;
use std::io;

// Return an HTML string of the table of contents
pub fn get_html(url: &str) -> String {
    task::block_on(async {
        let mut reader = surf::get(url).await.expect("Failed to fetch URL");

        if reader.status() != 200 {
            panic!("Website response was not OK");
        }

        reader.body_string().await.expect("Failed in parsing URL")
    })
}

// Return an HTML string of the table of contents
pub fn get_bytes(url: &str) -> io::Result<Vec<u8>> {
    Ok(task::block_on(async {
        let mut reader = surf::get(url).await.expect("Failed to fetch URL");

        if reader.status() != 200 {
            panic!("Website response was not OK");
        }

        reader.body_bytes().await.expect("Failed in parsing URL")
    }))
}
