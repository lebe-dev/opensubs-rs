pub mod search {
    use crate::domain::domain::SubtitleSearchResults;
    use crate::error::error::OperationError;
    use crate::parser::parser::get_search_results;

    /**
                sub_langs - subtitle languages. Example: rus,ara
                (Russian, Arabic)
                */
    pub async fn search_by_mask(client: &reqwest::Client, base_url: &str,
                                mask: &str, sub_langs: &str) ->
                                                Result<SubtitleSearchResults, OperationError> {
        info!("search subtitles by mask '{}'", mask);
        info!("language '{}'", sub_langs);

        let request_url = get_request_url(base_url, mask, sub_langs);

        debug!("request url:");
        debug!("{}", request_url);

        match client.get(&request_url).send().await {
            Ok(resp) => {
                let status: reqwest::StatusCode = resp.status();
                debug!("server response code: {}", status.as_str());

                if status == reqwest::StatusCode::OK {
                    let response_text = resp.text().await.unwrap();

                    trace!("---[SEARCH RESULTS]---");
                    trace!("{}", &response_text);
                    trace!("---[/SEARCH RESULTS]---");

                    match get_search_results(&response_text) {
                        Ok(search_results) => Ok(search_results),
                        Err(_) => Err(OperationError::Error)
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

    fn get_request_url(base_url: &str, search_mask: &str, language: &str) -> String {
        let sanitized_mask = search_mask.replace(" ", "+");
        format!("{}/ru/search2?MovieName={}&id=8&action=search&SubLanguageID=rus&SubLanguageID={}\
                &Season=&Episode=&SubSumCD=&Genre=&MovieByteSize=&MovieLanguage=&\
                MovieImdbRatingSign=1&MovieImdbRating=&MovieCountry=&MovieYearSign=1&\
                MovieYear=&MovieFPS=&SubFormat=&SubAddDate=&Uploader=&IDUser=&Translator=&\
                IMDBID=&MovieHash=&IDMovie=", base_url, sanitized_mask, language)
    }
}
