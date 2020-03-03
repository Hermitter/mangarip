use async_std::task;
use mangarip::book::*;
use mangarip::{Selector, Sorting};
use std::sync::{Arc, Mutex};

extern crate image;
use image::GenericImageView;
use tokio;
use tokio::prelude::*;

use std::thread::sleep;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let now = Instant::now();

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
    book2.download_all_chapters().await.unwrap();
    println!("Downloaded Chapters -> {:#?}", now.elapsed());

    // print image
    // let mut handlers = Vec::new();

    // for (c, chapter) in book2.chapters.iter().enumerate() {
    //     for (p, page) in chapter.borrow().pages.iter().enumerate() {
    //         let page = Arc::new(Mutex::new(page));
    //         let page_clone = page.clone();

    //         handlers.push(tokio::spawn(async move {
    //             page_clone;

    //             // create_image(
    //             //     ,
    //             //     &format!("./chapter/{}-{}.png", c, p),
    //             // );
    //         }));
    //     }
    // }

    // for handle in handlers {
    //     handle.await.unwrap();
    // }
    println!("Created Images -> {:#?}", now.elapsed());
}

async fn create_image(buffer: &[u8], output: &str) {
    let img = image::load_from_memory(buffer).unwrap();
    img.save(output).unwrap();
    println!("saved --> {}", output);
}
