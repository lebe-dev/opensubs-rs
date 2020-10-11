pub mod domain {
    use serde::{Deserialize, Serialize};

    pub type SubtitleSearchResults = Vec<SubtitleSearchResultItem>;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct SubtitleSearchResultItem {
        pub index: u16,

        pub title: String,

        pub season: u16,
        pub episode: u16
    }
}