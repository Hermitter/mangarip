use async_std::task;
use mangarip::url::*;
use regex::Regex;

/// Unreachable URL.
#[test]
#[should_panic]
fn fetch_404_url() {
    task::block_on(async {
        Request::new()
            .attempts(1)
            .fetch_as_string("https://github.com/Hermitter/mangarip/thispageisnotreal3014")
            .await
            .unwrap()
    });
}

/// Check if reachable URL returns 200 OK status.
#[test]
fn fetch_200_url() {
    task::block_on(async {
        Request::new()
            .attempts(10)
            .delay(3)
            .fetch_as_string("https://github.com/Hermitter/mangarip")
            .await
            .unwrap()
    });
}

#[test]
fn html_to_string() {
    task::block_on(async {
        let html = Request::new()
            .attempts(10)
            .fetch_as_string("https://github.com/Hermitter/mangarip")
            .await
            .unwrap();

        // test some common HTML tags
        assert!(Regex::new(r"<head").unwrap().is_match(&html));
        assert!(Regex::new(r"<html").unwrap().is_match(&html));
        assert!(Regex::new(r"<body").unwrap().is_match(&html));
        assert!(Regex::new(r"<div").unwrap().is_match(&html));
    });
}

#[test]
fn html_to_bytes() {
    task::block_on(async {
        // TODO: test if bytes are correct
        let bytes = Request::new()
            .attempts(10)
            .fetch_as_bytes("https://github.com/Hermitter/mangarip")
            .await
            .unwrap();
    });
}
