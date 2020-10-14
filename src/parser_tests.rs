#[cfg(test)]
mod parser_tests {
    use std::fs::File;
    use std::io::Read;

    use encoding::{DecoderTrap, Encoding};
    use encoding::all::WINDOWS_1251;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Logger, Root};
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs::filter::threshold::ThresholdFilter;
    use log::LevelFilter;

    use crate::parser::parser::parse_search_results;

    #[test]
    fn results_should_contain_search_result_items() {
        let logging_config = get_logging_config(LevelFilter::Debug);
        log4rs::init_config(logging_config).unwrap();
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

    fn get_html_content(filename: &str) -> String {
        let file_path = format!("tests/{}", filename);
        let mut file = File::open(file_path).expect("file not found");

        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("unable to read sample file");

        WINDOWS_1251.decode(&data, DecoderTrap::Strict)
            .expect("unable to get sample html data")
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
