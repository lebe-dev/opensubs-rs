#[cfg(test)]
mod search_tests {
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Logger, Root};
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs::filter::threshold::ThresholdFilter;
    use log::LevelFilter;

    use crate::auth::auth::login;
    use crate::BASE_URL;
    use crate::search::search::search_by_mask;

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
                match search_by_mask(&client, BASE_URL, "Midnight Gospel").await {
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

    fn get_logging_config(level: LevelFilter) -> Config {
        Config::builder()
            .appender(get_file_appender_definition(level))
            .logger(get_default_logger(level))
            .logger(Logger::builder().build("scraper", LevelFilter::Info))
            .logger(Logger::builder().build("html5ever", LevelFilter::Info))
            .logger(Logger::builder().build("selectors", LevelFilter::Info))
            .logger(Logger::builder().build("hyper", LevelFilter::Info))
            .logger(Logger::builder().build("mio", LevelFilter::Info))
            .build(
                Root::builder()
                    .appender("file")
                    .build(level)
            ).expect(&format!("unable to create log file 'parser.log'"))
    }

    fn get_file_appender_definition(level: LevelFilter) -> Appender {
        Appender::builder()
            .filter(Box::new(ThresholdFilter::new(level)))
            .build("file", Box::new(get_file_appender())
            )
    }

    fn get_file_appender() -> FileAppender {
        FileAppender::builder()
            .encoder(get_encoder())
            .build("parser.log")
            .unwrap()
    }

    fn get_encoder() -> Box<PatternEncoder> {
        Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} - {l} - [{M}] - {m}{n}"))
    }

    fn get_default_logger(level: LevelFilter) -> Logger {
        Logger::builder()
            .build("default", level)
    }
}
