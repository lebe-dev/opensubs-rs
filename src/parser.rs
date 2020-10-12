pub mod parser {
    use regex::Regex;
    use scraper::{Html, Selector};

    use crate::domain::domain::{SubtitleSearchResultItem, SubtitleSearchResults};
    use crate::error::error::OperationError;
    use crate::strip::strip::strip_html_tags;
    use crate::types::types::OperationResult;

    pub fn get_search_results(html: &str) -> OperationResult<SubtitleSearchResults> {
        info!("get search results from html");
        let mut results: SubtitleSearchResults = Vec::new();

        let document = Html::parse_fragment(html);

        let results_table_selector = Selector::parse("#search_results").unwrap();
        let title_col_selector = Selector::parse("td").unwrap();
        let title_details_url_selector = Selector::parse("a").unwrap();

        let series_pattern = Regex::new("\\[S(\\d{1,2})E(\\d{1,2})\\]").unwrap();
        let year_pattern = Regex::new(".*\\((\\d{4})\\).*").unwrap();

        let mut row_index: u16 = 1;

        match document.select(&results_table_selector).next() {
            Some(search_results_table) => {
                debug!("search results table found");
                let table_body_selector = Selector::parse("tbody").unwrap();

                match search_results_table.select(&table_body_selector).next() {
                    Some(table_body) => {
                        debug!("search results table body has been found");
                        let rows_selector = Selector::parse("tr").unwrap();

                        for row in table_body.select(&rows_selector) {
                            trace!("---[ROW]---");
                            trace!("{}", row.html());
                            trace!("---[/ROW]---");

                            let mut details_page_url: &str = "";

                            match row.select(&title_col_selector).skip(1).next() {
                                Some(title_col) => {
                                    match title_col.select(&title_details_url_selector).next() {
                                        Some(a_element) => {
                                            match a_element.value().attr("href") {
                                                Some(href) => details_page_url = href,
                                                None => {}
                                            }
                                        }
                                        None => {}
                                    }

                                    let title_row = strip_html_tags(&title_col.inner_html());

                                    let title_parts: Vec<&str> = title_row.split("\n").collect();

                                    let title = title_parts.get(0).unwrap();
                                    let year_part = *title_parts.get(1).unwrap();

                                    let mut year = String::from(year_part);

                                    if year_pattern.is_match(year_part) {
                                        let groups = year_pattern.captures_iter(year_part).next().unwrap();

                                        year = String::from(&groups[1]);
                                    }

                                    let merged_title = format!("{} ({})", title, year);

                                    info!("year '{}'", year_part);

                                    let mut season: u16 = 0;
                                    let mut episode: u16 = 0;

                                    match title_parts.get(2) {
                                        Some(series_info) => {
                                            info!("series info '{}'", series_info);

                                            if series_pattern.is_match(series_info) {
                                                let groups = series_pattern.captures_iter(series_info).next().unwrap();

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
                                        }
                                        None => debug!("no series info found")
                                    }

                                    let search_result_item = SubtitleSearchResultItem {
                                        index: row_index,
                                        title: merged_title.to_string(),
                                        details_url: details_page_url.to_string(),
                                        season,
                                        episode
                                    };

                                    results.push(search_result_item);

                                    row_index += 1;
                                }
                                None => error!("unable to get column with title")
                            }
                        }
                    }
                    None => error!("unable to find table body")
                }

                Ok(results)
            }
            None => {
                error!("unable to get search results table");
                Err(OperationError::HtmlParseError)
            }
        }
    }
}
