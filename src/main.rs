use async_std::task;
use mangarip::book::*;
use mangarip::{Selector, Sorting};

fn main() {
    let manganelo = Host {
        toc_sorting: Sorting::Descending,
        chapter_selector: Selector::Css(".row-content-chapter li a".to_owned()),
        page_selector: Selector::Regex(r#"src *= *"([^"]+/\d+\.(?:jpg|png))""#.to_owned()),
        chapter_url_append: "".to_owned(),
    };

    task::block_on(async {
        let book = Book::from("https://manganelo.com/manga/fk922312", &manganelo)
            .await
            .unwrap();

        println!("{:?}", book);
    });
}
