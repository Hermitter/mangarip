mod error;
mod host;
mod scrape;
mod web_util;

pub use error::*;
pub use scrape::*;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_chapter_scrape() {
//         println!("HELOOOOOOOO");

//         let scraper = Scraper::from("https://mangakakalot.com/manga/pj919819").unwrap();
//         // let chapters = scraper.get_chapters();

//         // println!("{:?}", chapters);

//         // let scraper = extract::request("https://mangakakalot.com/manga/pj919819");
//         // let chapters = scraper.get_chapters();
//         // let images = scraper.get_chapters();

//         // println!("{:?}", chapters);

//         assert_eq!(true, true);
//     }
// }
