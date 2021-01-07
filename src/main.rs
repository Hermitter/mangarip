use mangarip::book::{Book, Host};
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() {
    let timer = Instant::now();

    let url = "https://manganelo.com/manga/un921372";
    let host = Host::find_host(url).unwrap();
    let mut book = Book::new(url, &host).await.unwrap();
    book.scan().await;

    let timer = timer.elapsed();
    println!("{:#?}", book);
    println!("Time: {:#?}", timer);
}
