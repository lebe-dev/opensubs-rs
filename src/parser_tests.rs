#[cfg(test)]
mod parser_tests {
    use log::LevelFilter;

    use crate::parser::parser::{get_page_type, get_sub_download_url_from_page, PageType, parse_episode_page, parse_search_results};
    use crate::test_utils::test_utils::{get_html_content, get_logging_config};

    #[test]
    fn results_should_contain_search_result_items() {
        let content = get_html_content("series-search-results.html");

        match parse_search_results(&content) {
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
    fn parse_item_from_episode_page() {
        let content = get_html_content("season-page.html");

        let page_url: &str = "abc";

        match parse_episode_page(&content, page_url) {
            Ok(results) => {
                let item = results.first().unwrap();

                assert_eq!("Adventure Time Russian S10E04", item.title);
                assert_eq!(1, item.index);
                assert_eq!(10, item.season);
                assert_eq!(4, item.episode);
                assert_eq!(page_url, item.details_url);
            }
            Err(_) => panic!("results expected")
        }
    }

    #[test]
    fn parse_item_from_episode_page_subtitles_word_should_be_removed() {
        let content = get_html_content("season-page.html");

        match parse_episode_page(&content, "xyz") {
            Ok(results) => {
                let item = results.first().unwrap();
                assert_eq!("Adventure Time Russian S10E04", item.title);
            }
            Err(_) => panic!("results expected")
        }
    }

    #[test]
    fn parse_item_from_episode_page_season_and_episode_should_be_parsed_from_title() {
        let logging_config = get_logging_config(LevelFilter::Debug);
        log4rs::init_config(logging_config).unwrap();
        let content = get_html_content("season-page.html");

        match parse_episode_page(&content, "whatever") {
            Ok(results) => {
                let item = results.first().unwrap();
                assert_eq!(10, item.season);
                assert_eq!(4, item.episode);
            }
            Err(_) => panic!("results expected")
        }
    }

    #[test]
    fn parse_sub_download_url_from_episode_page() {
        let content = get_html_content("season-page.html");

        match get_sub_download_url_from_page(&content, "xyz") {
            Ok(download_url) => {
                match download_url {
                    Some(url) => assert_eq!("/en/subtitleserve/sub/7863206", url),
                    None => panic!("url expected")
                }
            }
            Err(_) => panic!("results expected")
        }
    }

    #[test]
    fn page_with_multiple_options() {
        let content = get_html_content("series-search-results.html");
        assert_eq!(get_page_type(&content), PageType::MultipleOptions)
    }

    #[test]
    fn page_with_single_option() {
        let content = get_html_content("episode-page.html");
        assert_eq!(get_page_type(&content), PageType::SingleOption)
    }
}
