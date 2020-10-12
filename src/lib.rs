#[macro_use]
extern crate log;
extern crate log4rs;

use crate::domain::domain::SubtitleSearchResults;
use crate::error::error::OperationError;
use crate::parser::parser::get_search_results;

mod domain;
mod parser;
mod error;
mod types;
mod parser_tests;
mod strip;
mod strip_tests;
mod search_tests;

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
        Ok(resp) => {
            let rsp_header = resp.headers();

            debug!("response Header: {:?}", rsp_header);

            let cookies = resp.cookies();

            for cookie in cookies {
                debug!("cookie: '{}' value '{}'", cookie.name(), cookie.value());
            }

            let status: reqwest::StatusCode = resp.status();

            debug!("status code '{}'", status);

            if status == reqwest::StatusCode::OK {
                let html = resp.text().await.unwrap();

                trace!("---[AUTH RESPONSE]---");
                trace!("{}", &html);
                trace!("---[/AUTH RESPONSE]---");

                Ok(())

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

/// Search subtitles by mask
///
/// You must provide `mask` and `sub_langs`.
///
/// # Examples
/// ```
/// use opensubs_rs::{search_by_mask, BASE_URL};
///
/// let client: reqwest::Client = reqwest::Client::builder()
///                                     .cookie_store(true)
///                                     .build()?;
/// // login operation here
///
/// match search_by_mask(&client, BASE_URL, "Midnight Gospel S01", "rus,gle").await {
///     Ok(search_results) => {}
///     Err(e) => {}
/// }
/// ```
/// Search subtitles by `mask
    /// `sub_langs` - subtitle languages. Example: rus,ara
    ///                 (Russian, Arabic)
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
                match  resp.text().await {
                    Ok(response_text) => {
                        trace!("---[SEARCH RESULTS]---");
                        trace!("{}", &response_text);
                        trace!("---[/SEARCH RESULTS]---");

                        match get_search_results(&response_text) {
                            Ok(search_results) => Ok(search_results),
                            Err(_) => Err(OperationError::Error)
                        }
                    }
                    Err(e) => {
                        error!("unable to get response text: {}", e);
                        Err(OperationError::Error)
                    }
                }

            } else {
                error!("error, response code was {}", status);
                Err(OperationError::Error)
            }
        }
        Err(e) => {
            error!("subtitles search error: {}", e);
            Err(OperationError::Error)
        }
    }
}

fn get_search_request_url(base_url: &str, search_mask: &str, language: &str) -> String {
    let sanitized_mask = search_mask.replace(" ", "+");
    format!("{}/ru/search2?MovieName={}&id=8&action=search&SubLanguageID=rus&SubLanguageID={}\
                &Season=&Episode=&SubSumCD=&Genre=&MovieByteSize=&MovieLanguage=&\
                MovieImdbRatingSign=1&MovieImdbRating=&MovieCountry=&MovieYearSign=1&\
                MovieYear=&MovieFPS=&SubFormat=&SubAddDate=&Uploader=&IDUser=&Translator=&\
                IMDBID=&MovieHash=&IDMovie=", base_url, sanitized_mask, language)
}


