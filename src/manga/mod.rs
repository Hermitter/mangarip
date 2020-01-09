mod book;
mod web_utils;
use url::Url;

pub fn request(url: &str) {
    // Check if URL is valid
    let scraper = match Url::parse(url).unwrap().host_str().unwrap() {
        "mangakakalot.com" => book::Scraper {
            chapter_selector: ".chapter-list .row span a",
            image_selector: "#vungdoc img",
            chapter_sort: book::Sorting::Descending,
        },
        "mangairo.com" => book::Scraper {
            chapter_selector: "",
            image_selector: "",
            chapter_sort: book::Sorting::Ascending,
        },
        "manganelo.com" => book::Scraper {
            chapter_selector: "",
            image_selector: "",
            chapter_sort: book::Sorting::Ascending,
        },
        _ => {
            panic!("Website is not supported");
        }
    };

    scraper.scrape(url);
}
