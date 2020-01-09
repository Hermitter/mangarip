use std::path::PathBuf;
use structopt::StructOpt;
mod manga;

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

    manga::request(&args.url);
}
