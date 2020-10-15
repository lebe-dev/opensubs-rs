#[cfg(test)]
mod search_tests {
    use log::LevelFilter;

    use crate::{BASE_URL, login, search_serial_season};
    use crate::test_utils::test_utils::get_logging_config;

    const LOGIN: &str = "CHANGE-ME";
    const PASSWORD: &str = "CHANGE-ME";

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
                    LOGIN, PASSWORD).await {
            Ok(_) => {
                match search_serial_season(&client, BASE_URL,
                                     "Midnight Gospel", "rus", 1).await {
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
