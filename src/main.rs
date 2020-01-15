use mangarip;
use mangarip::Fetch;
use std::sync::mpsc;
use std::thread;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mangarip", about = "A web scraper tool for downloading manga")]
struct Cli {
    url: String, // manga overview page

    #[structopt(short, long)]
    format: Option<String>, // PDF or image

    #[structopt(short = "b", long = "book")]
    compile_into_book: bool, // store all images into a single directory (meant for PDF)
}

fn main() {
    let args = Cli::from_args();

    let scraper = mangarip::Scraper::from("https://mangakakalot.com/manga/pj919819").unwrap();

    let x = scraper.get_chapters(0, scraper.chapter_urls.len() as u32, |i| {
        println!("Chapter {} Finished", i);
    });
}
