#[cfg(test)]
mod search_tests {
    use reqwest::Client;

    use crate::{BASE_URL, get_download_url_from_page, search_by_mask, search_serial_episode, search_serial_season};

    const SEARCH_MASK: &str = "Midnight Gospel";

    #[tokio::test]
    async fn search_movie_with_multi_results() {
        let client = get_client();

        match search_by_mask(
            &client, BASE_URL,
            "tideland", "rus,eng"
        ).await {
            Ok(results) => {
                println!("{:?}", results);
                assert!(results.len() > 1);

                let first_movie = results.get(0).unwrap();

                assert_eq!("Tideland (2005)", first_movie.title);
            },
            Err(_) => panic!("search results expected")
        }
    }

    #[tokio::test]
    async fn search_serial_episode_with_one_result() {
        let client = get_client();

        match search_serial_episode(
            &client, BASE_URL,
            SEARCH_MASK, "rus", 1, 2
        ).await {
            Ok(results) => assert_eq!(1, results.len()),
            Err(_) => panic!("search results expected")
        }
    }

    #[tokio::test]
    async fn search_serial_episode_with_multi_results() {
        let client = get_client();

        match search_serial_episode(
            &client, BASE_URL,
            SEARCH_MASK, "rus,eng", 1, 2
        ).await {
            Ok(results) => {
                println!("{:?}", results);
                assert!(results.len() > 1)
            },
            Err(_) => panic!("search results expected")
        }
    }

    #[tokio::test]
    async fn result_should_contain_relative_url() {
        let client = get_client();

        let url = format!("{}/en/subtitles/8314554/midnight-sun-ko", BASE_URL);

        match get_download_url_from_page(&client, &url).await {
            Ok(url) => assert_eq!("/en/subtitleserve/sub/8314554", url.unwrap()),
            Err(_) => panic!("search results expected")
        }
    }

    #[tokio::test]
    async fn result_should_contain_series_episode_search_results() {
        let client = get_client();

        match search_serial_episode(&client, BASE_URL, SEARCH_MASK,
                                    "rus", 1, 2).await {
            Ok(search_results) => {
                assert!(search_results.len() > 0);
                println!("{:?}", &search_results);
            }
            Err(_) => panic!("search results expected")
        }
    }

    #[tokio::test]
    async fn result_should_contain_series_season_search_results() {
        let client = get_client();

        match search_serial_season(&client, BASE_URL,
                                   SEARCH_MASK, "rus", 1).await {
            Ok(search_results) => {
                assert!(search_results.len() > 0);
                println!("{:?}", &search_results);
            }
            Err(_) => panic!("search results expected")
        }
    }

    fn get_client() -> Client {
        reqwest::Client::builder()
            .user_agent("Google Chrome")
            .connection_verbose(true)
            .cookie_store(true)
            .build()
            .unwrap()
    }
}
