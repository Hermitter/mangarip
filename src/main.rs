use mangarip;
use mangarip::Fetch;
use std::path::PathBuf;
use structopt::StructOpt;

extern crate printpdf;

// imports the `image` library with the exact version that we are using
use printpdf::*;

use std::convert::From;
use std::fs::File;

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

    let mut scraper = mangarip::Scraper::from(&args.url).unwrap();
    scraper.scan_toc().unwrap();
    scraper.get_chapter(0);

    // for chapter in chapters {
    //     println!
    // }
}
