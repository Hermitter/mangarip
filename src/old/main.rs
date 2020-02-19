use mangarip;
use structopt::StructOpt;

extern crate image;
use tokio;

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

    println!("Hello World");
    // let scraper = mangarip::Scraper::from(&args.url).await.unwrap();
    // let chapter = scraper.get_chapter(0).await.unwrap();
}

fn create_image(buffer: &[u8], output: &str) {
    let img = image::load_from_memory(buffer).unwrap();
    img.save(output).unwrap();
    println!("saved --> {}", output);
}
