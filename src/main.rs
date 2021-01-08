use mangarip::book::{Book, Host};
use std::time::Instant;

#[tokio::main]
async fn main() {
    let timer = Instant::now();

    let url = "https://manganelo.com/manga/un921372"; // < 30 chapters
                                                      // let url = "https://manganelo.com/manga/komisan_wa_komyushou_desu"; // 200+ chapters

    let host = Host::find_host(url).unwrap();
    let book = Book::new(url, &host)
        .scan_chapters()
        .await
        .unwrap()
        .scan_images()
        .await;

    let timer = timer.elapsed();
    println!("{:#?}", book);
    println!("Time: {:#?}", timer);
}
