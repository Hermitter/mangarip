mod lib;
use lib::book::{host::Host, *};
use lib::{Selector, Sorting};
use std::sync::{Arc, Mutex};

extern crate image;
use tokio;

use std::time::Instant;

#[tokio::main]
async fn main() {
    let now = Instant::now();

    // Mangaelo
    let manganelo_com = Host {
        toc_sorting: Sorting::Descending,
        chapter_selector: Selector::Css(".row-content-chapter li a".to_owned()),
        image_selector: Selector::Regex(r#"src *= *"([^"]+/\d+\.(?:jpg|png))""#.to_owned()),
        chapter_url_append: None,
    };

    let mut book = Book::new("https://manganelo.com/manga/pj919819", &manganelo_com)
        .await
        .unwrap();

    book.scan().await;

    println!("{:#?}", book.chapters);
    println!("Created Images -> {:#?}", now.elapsed());
}
