pub mod domain {
    use serde::{Deserialize, Serialize};

    pub type SubtitleSearchResults = Vec<SubtitleSearchResultItem>;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct SubtitleSearchResultItem {
        pub index: u8,

        pub title: String,

        /**
        Relative url to movie page
        */
        pub details_url: String,

        pub season: u8,
        pub episode: u16
    }
}
