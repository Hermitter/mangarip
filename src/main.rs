use async_std::task;
use mangarip::book::*;
use mangarip::{Selector, Sorting};
use std::sync::Arc;

extern crate image;
use image::GenericImageView;
use tokio;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    // Mangaelo
    // let manganelo = Host {
    //     toc_sorting: Sorting::Descending,
    //     chapter_selector: Selector::Css(".row-content-chapter li a".to_owned()),
    //     page_selector: Selector::Regex(r#"src *= *"([^"]+/\d+\.(?:jpg|png))""#.to_owned()),
    //     chapter_url_append: "".to_owned(),
    // };

    // let mut book1 = Book::from("https://manganelo.com/manga/fk922312", &manganelo)
    //     .await
    //     .unwrap();
    // book1.download_chapter(0).await;

    // Mangakakalot
    let mangakakalot = Host {
        toc_sorting: Sorting::Descending,
        chapter_selector: Selector::Css(".chapter-list .row span a".to_owned()),
        page_selector: Selector::Regex(r#"src *= *"([^"]+/\d+\.(?:jpg|png))""#.to_owned()),
        chapter_url_append: "".to_owned(),
    };

    let mut book2 = Book::from("https://mangakakalot.com/manga/pj919819", &mangakakalot)
        .await
        .unwrap();
    book2.download_chapter(0).await;
}

fn create_image(buffer: &[u8], output: &str) {
    let img = image::load_from_memory(buffer).unwrap();
    img.save(output).unwrap();
    println!("saved --> {}", output);
}
