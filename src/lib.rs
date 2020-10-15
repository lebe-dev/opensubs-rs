#[macro_use]
extern crate log;
extern crate log4rs;

use crate::domain::domain::SubtitleSearchResults;
use crate::error::error::OperationError;
use crate::parser::parser::{get_page_type, get_sub_download_url_from_page, PageType, parse_episode_page, parse_series_search_results};
use crate::types::types::{OperationResult, OptionResult};

mod domain;
mod parser;
mod error;
mod types;
mod parser_tests;
mod strip;
mod strip_tests;
mod search_tests;
mod test_utils;

/// Open subtitles site url
pub const BASE_URL: &str = "https://www.opensubtitles.org";

pub async fn search_by_mask(client: &reqwest::Client, base_url: &str,
                            mask: &str, sub_langs: &str) -> OperationResult<SubtitleSearchResults> {
    info!("search subtitles by mask '{}'", mask);
    info!("- languages: '{}'", sub_langs);

    let request_url = get_default_search_url(base_url, mask, sub_langs);

    fetch_and_parse(
        client, &request_url,
        parse_series_search_results,
        html_parse_error_func_with_two_args
    ).await
}

pub async fn search_serial_season(client: &reqwest::Client, base_url: &str,
                                  mask: &str, sub_langs: &str, season: u8) ->
                                                          OperationResult<SubtitleSearchResults> {
    info!("search series subtitles by mask '{}'", mask);
    info!("- season '{}'", season);
    info!("- sub langs '{}'", sub_langs);

    let request_url = get_serial_season_search_url(base_url, mask, sub_langs, season);

    fetch_and_parse(
        client, &request_url,
        parse_series_search_results, html_parse_error_func_with_two_args
    ).await
}

pub async fn search_serial_episode(client: &reqwest::Client, base_url: &str,
                                  mask: &str, sub_langs: &str, season: u8, episode: u16) ->
                                  OperationResult<SubtitleSearchResults> {
    info!("search series subtitles by mask '{}'", mask);
    info!("- season '{}'", season);
    info!("- episode '{}'", episode);
    info!("- sub langs '{}'", sub_langs);

    let request_url = get_serial_episode_search_url(
        base_url, mask, sub_langs, season, episode
    );

    fetch_and_parse(
        client, &request_url,
        parse_series_search_results, parse_episode_page
    ).await
}

pub async fn get_download_url_from_page(client: &reqwest::Client,
                                        page_url: &str) -> OptionResult<String> {
    info!("get subtitles download url from page '{}'", page_url);
    fetch_and_parse(client, &page_url,
                    html_parse_error_func,
                    get_sub_download_url_from_page).await
}

fn get_default_search_url(base_url: &str, search_mask: &str, sub_langs: &str) -> String {
    let sanitized_mask = search_mask.replace(" ", "+");

    format!(
        "{}/en/search/sublanguageid-{}/moviename-{}",
        base_url, sub_langs, sanitized_mask
    )
}

fn get_serial_season_search_url(base_url: &str, search_mask: &str,
                                sub_langs: &str, season: u8) -> String {

    let sanitized_mask = search_mask.replace(" ", "+");

    format!(
        "{}/en/search/sublanguageid-{}/moviename-{}/season-{}/SearchOnlyTVSeries-on",
        base_url, sub_langs, sanitized_mask, season
    )
}

fn get_serial_episode_search_url(base_url: &str, search_mask: &str,
                                 sub_langs: &str, season: u8, episode: u16) -> String {

    let sanitized_mask = search_mask.replace(" ", "+");

    format!(
        "{}/en/search/sublanguageid-{}/moviename-{}/season-{}/episode-{}/SearchOnlyTVSeries-on",
        base_url, sub_langs, sanitized_mask, season, episode
    )
}

async fn fetch_and_parse<R>(
    client: &reqwest::Client, url: &str,
    multi_option_parser: impl Fn(&str) -> OperationResult<R>,
    single_option_parser: impl Fn(&str, &str) -> OperationResult<R>
) -> OperationResult<R> {
    debug!("request url:");
    debug!("'{}'", url);

    match client.get(url).send().await {
        Ok(resp) => {
            let status: reqwest::StatusCode = resp.status();
            debug!("server response code: {}", status.as_str());

            if status == reqwest::StatusCode::OK {
                match resp.text().await {
                    Ok(response_text) => {
                        trace!("---[SEARCH RESULTS]---");
                        trace!("{}", &response_text);
                        trace!("---[/SEARCH RESULTS]---");

                        match get_page_type(&response_text) {
                            PageType::MultipleOptions => multi_option_parser(&response_text),
                            PageType::SingleOption => single_option_parser(&response_text, &url)
                        }
                    }
                    Err(e) => {
                        error!("unable to get response text: {}", e);
                        Err(OperationError::Error)
                    }
                }

            } else {
                error!("unexpected server status code: {}", status);
                Err(OperationError::Error)
            }
        }
        Err(e) => {
            error!("unable to get data from url: {}", e);
            Err(OperationError::Error)
        }
    }
}

fn html_parse_error_func<R>(arg: &str) -> OperationResult<R> {
    error!("unexpected branch: {}", arg);
    Err(OperationError::HtmlParseError)
}

fn html_parse_error_func_with_two_args<R>(arg1: &str, arg2: &str) -> OperationResult<R> {
    error!("unexpected branch: {}", arg1);
    error!("{}", arg2);
    Err(OperationError::HtmlParseError)
}
