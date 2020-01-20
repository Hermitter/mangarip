use futures::join;
use mangarip;
use structopt::StructOpt;
use tokio::prelude::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "mangarip", about = "A web scraper tool for downloading manga")]
struct Cli {
    url: String, // manga overview page

    #[structopt(short, long)]
    format: Option<String>, // PDF or image

    #[structopt(short = "b", long = "book")]
    compile_into_book: bool, // store all images into a single directory (meant for PDF)
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();

    let scraper = mangarip::Scraper::from("https://mangakakalot.com/manga/pj919819")
        .await
        .unwrap();

    let x = scraper.get_chapter(0);
    let y = scraper.get_chapter(1);
    let z = scraper.get_chapter(2);

    join!(x, y, z);
}
