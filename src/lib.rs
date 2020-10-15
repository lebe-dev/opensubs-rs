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

    fetch_and_parse(client, &request_url, parse_series_search_results).await
}

pub async fn get_download_url_from_page(client: &reqwest::Client,
                                        page_url: &str) -> OptionResult<String> {
    info!("get subtitles download url from page '{}'", page_url);
    fetch_and_parse(client, &page_url, get_sub_download_url_from_page).await
}

fn get_serial_season_search_url(base_url: &str, search_mask: &str,
                                sub_langs: &str, season: u8) -> String {

    let sanitized_mask = search_mask.replace(" ", "+");

    format!(
        "{}/en/search/sublanguageid-{}/moviename-{}/season-{}/SearchOnlyTVSeries-on",
        base_url, sub_langs, sanitized_mask, season
    )
}

async fn fetch_and_parse<R>(client: &reqwest::Client, url: &str,
                            parser_func: impl Fn(&str) -> OperationResult<R>) -> OperationResult<R> {
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

                        match parser_func(&response_text) {
                            Ok(parse_results) => Ok(parse_results),
                            Err(_) => {
                                error!("unable to parse data");
                                Err(OperationError::HtmlParseError)
                            }
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


