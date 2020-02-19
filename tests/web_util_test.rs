use async_std::task;
use mangarip::*;

/// Unreachable URL.
#[test]
#[should_panic]
fn fetch_404_url() {
    task::block_on(async {
        web_util::get_html("https://www.google.com/thispageisnotreal3014")
            .await
            .unwrap();
    });
}

/// Check if reachable URL returns 200 OK status.
#[test]
fn fetch_200_url() {
    task::block_on(async {
        web_util::get_html("https://www.google.com").await.unwrap();
    });
}
