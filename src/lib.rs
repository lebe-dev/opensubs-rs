#[macro_use]
extern crate log;
extern crate log4rs;

use crate::domain::domain::SubtitleSearchResults;
use crate::error::error::OperationError;
use crate::search::search::search_by_mask;

mod domain;
mod parser;
mod error;
mod types;
mod parser_tests;
mod strip;
mod strip_tests;
mod auth;
mod search;
mod search_tests;

pub const BASE_URL: &str = "https://www.opensubtitles.org";

/// Search subtitles by mask
///
/// You must provide `mask` and `sub_langs`.
///
/// # Examples
/// ```
/// use opensubs_rs::search_subs_by_mask;
///
/// // It will search with mask 'Midnight Gospel S01' for Russian and Irish languages
/// search_subs_by_mask(&client, "Midnight Gospel S01", "rus,gle");
/// ```
pub async fn search_subs_by_mask(client: &reqwest::Client, mask: &str, sub_langs: &str) ->
                                            Result<SubtitleSearchResults, OperationError> {
    match search_by_mask(&client, BASE_URL, mask, sub_langs).await {
        Ok(search_results) => Ok(search_results),
        Err(e) => Err(e)
    }
}


