#[cfg(test)]
mod search_tests {
    use crate::{BASE_URL, get_download_url_from_page, search_serial_season};

    #[tokio::test]
    async fn result_should_contain_relative_url() {
        let client = reqwest::Client::builder()
            .user_agent("Google Chrome")
            .connection_verbose(true)
            .cookie_store(true)
            .build()
            .unwrap();

        match get_download_url_from_page(
            &client, "https://www.opensubtitles.org/en/subtitles/8314554/midnight-sun-ko"
        ).await {
            Ok(url) =>
                assert_eq!(
                    "/en/subtitleserve/sub/8314554",
                    url.unwrap()
                ),
            Err(_) => panic!("search results expected")
        }
    }

    #[tokio::test]
    async fn result_should_contain_series_search_results() {
        let client = reqwest::Client::builder()
            .user_agent("Google Chrome")
            .connection_verbose(true)
            .cookie_store(true)
            .build()
            .unwrap();

        match search_serial_season(&client, BASE_URL,
                                   "Midnight Gospel", "rus", 1).await {
            Ok(search_results) => {
                assert!(search_results.len() > 0);
                println!("{:?}", &search_results);
            }
            Err(_) => panic!("search results expected")
        }
    }
}
