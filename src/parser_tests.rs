#[cfg(test)]
mod parser_tests {
    use log::LevelFilter;

    use crate::parser::parser::{get_sub_download_url_from_page, parse_series_search_results};
    use crate::test_utils::test_utils::{get_html_content, get_logging_config};

    #[test]
    fn results_should_contain_search_result_items() {
        let content = get_html_content("series-search-results.html");

        match parse_series_search_results(&content) {
            Ok(search_results) => {
                println!("{:?}", search_results);

                assert_eq!(search_results.len(), 40);

                let first_result = search_results.first()
                                                .expect("unable to get first search result");

                assert_eq!(first_result.index, 1);
                assert_eq!(first_result.title, "\"Adventure Time\" Bonnibel Bubblegum (2017)");
                assert_eq!(first_result.details_url, "https://www.opensubtitles.org/en/subtitles/7863206/adventure-time-bonnibel-bubblegum-ru");
                assert_eq!(first_result.season, 10);
                assert_eq!(first_result.episode, 4);
            }
            Err(_) => panic!("results expected")
        }
    }

    #[test]
    fn parse_sub_download_url_from_episode_page() {
        let logging_config = get_logging_config(LevelFilter::Debug);
        log4rs::init_config(logging_config).unwrap();
        let content = get_html_content("episode-page.html");

        match get_sub_download_url_from_page(&content) {
            Ok(download_url) => {
                match download_url {
                    Some(url) => assert_eq!("/en/subtitleserve/sub/7863206", url),
                    None => panic!("url expected")
                }
            }
            Err(_) => panic!("results expected")
        }
    }
}
