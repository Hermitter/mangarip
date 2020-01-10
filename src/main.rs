use mangarip;
use std::path::PathBuf;
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

    let scraper = mangarip::Scraper::from(&args.url).unwrap();
    let chapters = scraper.get_chapters();
    println!("{:?}", chapters);

    // println!("{}", mangarip::X);
    // mangarip::X //::request(&args.url);
}
