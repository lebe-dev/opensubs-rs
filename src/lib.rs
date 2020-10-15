#[macro_use]
extern crate log;
extern crate log4rs;

use crate::domain::domain::SubtitleSearchResults;
use crate::error::error::OperationError;
use crate::parser::parser::{get_sub_download_url_from_page, parse_series_search_results};
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

/// Opensubtitles site url
pub const BASE_URL: &str = "https://www.opensubtitles.org";

/// Login to opensubtitles.org
///
/// # Examples
/// ```
/// use opensubs_rs::{BASE_URL, login};
///
/// let client: reqwest::Client = reqwest::Client::builder()
///             .cookie_store(true)
///             .build().unwrap();
///
/// login(&client, BASE_URL, "username", "supppaPazzWourd");
/// ```
pub async fn login(client: &reqwest::Client, base_url: &str,
                   login: &str, password: &str) -> Result<(), Box<OperationError>> {
    info!("login to '{}'", base_url);

    let params = [
        ("a", "login"),
        ("redirect", "/ru"),
        ("user", login),
        ("password", password),
        ("remember", "on"),
    ];

    let url = format!("{}/ru/login/redirect-%7Cru", base_url);

    match client.post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send().await {
        Ok(response) => {
            let response_header = response.headers();

            debug!("response Header: {:?}", response_header);

            let cookies = response.cookies();

            for cookie in cookies {
                debug!("cookie: '{}' value '{}'", cookie.name(), cookie.value());
            }

            let status: reqwest::StatusCode = response.status();

            debug!("status code '{}'", status);

            if status == reqwest::StatusCode::OK {
                match response.text().await {
                    Ok(html) => {
                        trace!("---[AUTH RESPONSE]---");
                        trace!("{}", &html);
                        trace!("---[/AUTH RESPONSE]---");

                        Ok(())
                    }
                    Err(e) => {
                        error!("unable to get response text: '{}'", e);
                        Err(Box::from(OperationError::Error))
                    }
                }

            } else {
                error!("error, response code was {}", status);
                Err(Box::from(OperationError::Authentication))
            }
        }
        Err(e) => {
            error!("authentication error, unable to connect: '{}'", e);
            Err(Box::from(OperationError::Error))
        }
    }
}

pub async fn search_serial_season(client: &reqwest::Client, base_url: &str,
                                  mask: &str, sub_langs: &str, season: u8) ->
                                                          OperationResult<SubtitleSearchResults> {
    info!("search series subtitles by mask '{}'", mask);
    info!("- season '{}'", season);
    info!("- sub langs '{}'", sub_langs);

    let request_url = get_serial_season_search_url(base_url, mask, sub_langs, season);

    debug!("request url:");
    debug!("{}", request_url);

    match client.get(&request_url).send().await {
        Ok(resp) => {
            let status: reqwest::StatusCode = resp.status();
            debug!("server response code: {}", status.as_str());

            if status == reqwest::StatusCode::OK {
                match resp.text().await {
                    Ok(response_text) => {
                        trace!("---[SEARCH RESULTS]---");
                        trace!("{}", &response_text);
                        trace!("---[/SEARCH RESULTS]---");

                        match parse_series_search_results(&response_text) {
                            Ok(search_results) => Ok(search_results),
                            Err(_) => Err(OperationError::Error)
                        }
                    }
                    Err(e) => {
                        error!("unable to get response text: {}", e);
                        Err(OperationError::Error)
                    }
                }

            } else { Err(OperationError::Error) }
        }
        Err(e) => {
            error!("subtitles search error: {}", e);
            Err(OperationError::Error)
        }
    }
}

pub async fn get_download_url_from_page(client: &reqwest::Client,
                                        base_url: &str, page_url: &str) -> OptionResult<String> {
    info!("get subtitles download url from page '{}'", page_url);

    match client.get(page_url).send().await {
        Ok(resp) => {
            let status: reqwest::StatusCode = resp.status();
            debug!("server response code: {}", status.as_str());

            if status == reqwest::StatusCode::OK {
                match resp.text().await {
                    Ok(response_text) => {
                        trace!("---[SEARCH RESULTS]---");
                        trace!("{}", &response_text);
                        trace!("---[/SEARCH RESULTS]---");

                        match get_sub_download_url_from_page(&response_text, base_url) {
                            Ok(download_url) => Ok(download_url),
                            Err(_) => Err(OperationError::Error)
                        }
                    }
                    Err(e) => {
                        error!("unable to get response text: {}", e);
                        Err(OperationError::Error)
                    }
                }

            } else { Err(OperationError::Error) }
        }
        Err(e) => {
            error!("url get error: {}", e);
            Err(OperationError::Error)
        }
    }
}

fn get_serial_season_search_url(base_url: &str, search_mask: &str,
                                sub_langs: &str, season: u8) -> String {

    let sanitized_mask = search_mask.replace(" ", "+");

    format!(
        "{}/en/search/sublanguageid-{}/moviename-{}/season-{}/SearchOnlyTVSeries-on",
        base_url, sub_langs, sanitized_mask, season
    )
}

/// Search subtitles by mask
///
/// You must provide `mask` and `sub_langs`, subtitle languages. Example: rus,ara (Russian, Arabic)
///
pub async fn search_by_mask(client: &reqwest::Client, base_url: &str,
                            mask: &str, sub_langs: &str) ->
                            Result<SubtitleSearchResults, OperationError> {
    info!("search subtitles by mask '{}'", mask);
    info!("language '{}'", sub_langs);

    let request_url = get_search_request_url(base_url, mask, sub_langs);

    debug!("request url:");
    debug!("{}", request_url);

    match client.get(&request_url).send().await {
        Ok(resp) => {
            let status: reqwest::StatusCode = resp.status();
            debug!("server response code: {}", status.as_str());

            if status == reqwest::StatusCode::OK {
                match resp.text().await {
                    Ok(response_text) => {
                        trace!("---[SEARCH RESULTS]---");
                        trace!("{}", &response_text);
                        trace!("---[/SEARCH RESULTS]---");

                        match parse_series_search_results(&response_text) {
                            Ok(search_results) => Ok(search_results),
                            Err(_) => Err(OperationError::Error)
                        }
                    }
                    Err(e) => {
                        error!("unable to get response text: {}", e);
                        Err(OperationError::Error)
                    }
                }

            } else { Err(OperationError::Error) }
        }
        Err(e) => {
            error!("subtitles search error: {}", e);
            Err(OperationError::Error)
        }
    }
}

fn get_search_request_url(base_url: &str, search_mask: &str, language: &str) -> String {
    let sanitized_mask = search_mask.replace(" ", "+");
    // https://www.opensubtitles.org/en/search/sublanguageid-rus,eng/moviename-adventure+time/season-2/SearchOnlyTVSeries-on
    format!("{}/ru/search2?MovieName={}&id=8&action=search&SubLanguageID=rus&SubLanguageID={}\
                &Season=&Episode=&SubSumCD=&Genre=&MovieByteSize=&MovieLanguage=&\
                MovieImdbRatingSign=1&MovieImdbRating=&MovieCountry=&MovieYearSign=1&\
                MovieYear=&MovieFPS=&SubFormat=&SubAddDate=&Uploader=&IDUser=&Translator=&\
                IMDBID=&MovieHash=&IDMovie=", base_url, sanitized_mask, language)
}


