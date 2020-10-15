#[cfg(test)]
mod search_tests {
    use log::LevelFilter;

    use crate::{BASE_URL, login, search_by_mask};
    use crate::test_utils::test_utils::get_logging_config;

    #[ignore]
    #[tokio::test]
    async fn result_should_contain_search_results() {
        let logging_config = get_logging_config(LevelFilter::Debug);
        log4rs::init_config(logging_config).unwrap();

        let client = reqwest::Client::builder()
            .user_agent("Google Chrome")
            .connection_verbose(true)
            .cookie_store(true)
            .build()
            .unwrap();

        match login(&client, BASE_URL,
                    "CHANGE-ME", "CHANGE-ME").await {
            Ok(_) => {
                match search_by_mask(&client, BASE_URL,
                                     "Midnight Gospel", "rus").await {
                    Ok(search_results) => {
                        assert!(search_results.len() > 0);
                        println!("{:?}", &search_results);
                    }
                    Err(_) => panic!("search results expected")
                }
            }
            Err(_) => panic!("auth success expected")
        }
    }
}
