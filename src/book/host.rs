use crate::{Selector, Sorting};

/// Information needed to support a new manga website.
#[derive(Debug)]
pub struct Host {
    /// Describes how the chapter list is sorted in the table of contents.
    pub toc_sorting: Sorting,
    /// Selector for each chapter url in the table of contents.
    pub chapter_selector: Selector,
    /// Selector for each image url in a chapter.
    pub page_selector: Selector,
    /// String to append at the end of a chapter's URL.
    pub chapter_url_append: String,
}

impl<'a> Host {
    // Creates a new instance of Host with default values for generic sites.
    // pub fn default() -> Host {
    //     Host {
    //         toc_sorting: Sorting::Descending,
    //         chapter_selector: Selector::Regex("".to_owned()),
    //         page_selector: Selector::Regex(r#"src *= *"([^"]+/\d+\.(?:jpg|png))""#.to_owned()),
    //         chapter_url_append: "".to_owned(),
    //     }
    // }
}
