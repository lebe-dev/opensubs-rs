pub mod parser {
    use regex::Regex;
    use scraper::{ElementRef, Html, Selector};

    use crate::domain::domain::{SubtitleSearchResultItem, SubtitleSearchResults};
    use crate::error::error::OperationError;
    use crate::strip::strip::strip_html_tags;
    use crate::types::types::{OperationResult, OptionResult};

    pub fn parse_series_search_results(html: &str) -> OperationResult<SubtitleSearchResults> {
        info!("parse search results from html");
        let mut results: SubtitleSearchResults = Vec::new();

        let document = Html::parse_fragment(html);

        let title_col_selector = Selector::parse("td").unwrap();
        let title_details_url_selector = Selector::parse("a").unwrap();

        let series_pattern = Regex::new("\\[S(\\d{1,2})E(\\d{1,2})\\]").unwrap();
        let year_pattern = Regex::new(".*\\((\\d{4})\\).*").unwrap();

        let mut row_index: u8 = 1;

        let results_table_selector = Selector::parse("#search_results").unwrap();

        match document.select(&results_table_selector).next() {
            Some(search_results_table) => {
                debug!("search results table found");
                let table_body_selector = Selector::parse("tbody").unwrap();

                match search_results_table.select(&table_body_selector).next() {
                    Some(table_body) => {
                        debug!("search results table body has been found");
                        let rows_selector = Selector::parse("tr.change").unwrap();

                        for row in table_body.select(&rows_selector) {
                            match get_search_item_from_row(
                                row_index, &row, &title_col_selector,
                                &title_details_url_selector, &year_pattern, &series_pattern
                            ) {
                                Ok(search_result_item) => {
                                    results.push(search_result_item);
                                    row_index += 1;
                                },
                                Err(e) =>
                                    error!("unable to extract search result item from row: {}", e)
                            }
                        }

                        Ok(results)
                    }
                    None => {
                        Err(OperationError::HtmlParseError)
                    }
                }
            }
            None => {
                Err(OperationError::HtmlParseError)
            }
        }
    }

    pub fn get_sub_download_url_from_episode_page(html: &str, base_url: &str) -> OptionResult<String> {
        let mut result: OptionResult<String> = Ok(None);

        let a_element_selector = Selector::parse("a.bt-dwl").unwrap();

        let document = Html::parse_fragment(html);

        match document.select(&a_element_selector).next() {
            Some(a_element) => {
                match a_element.value().attr("href") {
                    Some(href) => {
                        let url = format!("{}{}", base_url, href);
                        result = Ok(Some(url))
                    }
                    None => {
                        warn!("<a> tag doesn't have 'href' attribute. unexpected html");
                        result = Err(OperationError::HtmlParseError)
                    }
                }
            }
            None => {
                error!("unable to parse subtitle download url");
                result = Err(OperationError::HtmlParseError)
            }
        }

        return result
    }

    fn get_search_item_from_row(row_index: u8, row: &ElementRef,
                                title_col_selector: &Selector,
                                title_details_url_selector: &Selector,
                                year_pattern: &Regex, series_pattern: &Regex) ->
                                                 Result<SubtitleSearchResultItem, OperationError> {
        trace!("---[ROW]---");
        trace!("{}", row.html());
        trace!("---[/ROW]---");

        let mut result: OperationResult<SubtitleSearchResultItem> = Err(OperationError::Error);

        let mut details_page_url: &str = "";

        let mut title: String = String::new();

        match row.select(&title_col_selector).next() {
            Some(title_col) => {
                match title_col.select(&title_details_url_selector).next() {
                    Some(a_element) => {
                        title = a_element.text().next().unwrap()
                                         .replace("\n", " ").to_string();

                        match a_element.value().attr("href") {
                            Some(href) => details_page_url = href,
                            None => {}
                        }
                    }
                    None => {}
                }

                let title_row = strip_html_tags(&title_col.inner_html());
                debug!("TITLE ROW: '{}'", title_row);

                if title_row.len() > 1 {
                    let mut year = String::new();

                    match year_pattern.find(&title_row) {
                        Some(year_match) => {
                            year = title_row[year_match.start()+1..year_match.end()-1].to_string();
                        }
                        None => {}
                    }

                    info!("year '{}'", year);

                    let mut season: u8 = 0;
                    let mut episode: u16 = 0;

                    if series_pattern.is_match(&title_row) {
                        let groups = series_pattern.captures_iter(&title_row).next().unwrap();

                        match String::from(&groups[1]).parse() {
                            Ok(value) => season = value,
                            Err(e) =>
                                error!("unable to get season value: {}", e)
                        }

                        match String::from(&groups[2]).parse() {
                            Ok(value) => episode = value,
                            Err(e) =>
                                error!("unable to get episode value: {}", e)
                        }
                    }

                    let search_result_item = SubtitleSearchResultItem {
                        index: row_index,
                        title,
                        details_url: details_page_url.to_string(),
                        season,
                        episode
                    };

                    result = Ok(search_result_item);
                }
            }
            None => error!("unable to get column with title")
        }

        result
    }
}
