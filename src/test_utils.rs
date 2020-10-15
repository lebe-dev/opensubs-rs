#[cfg(test)]
pub mod test_utils {
    use std::fs::File;
    use std::io::Read;

    use encoding::{DecoderTrap, Encoding};
    use encoding::all::WINDOWS_1251;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Logger, Root};
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs::filter::threshold::ThresholdFilter;
    use log::LevelFilter;

    pub fn get_logging_config(level: LevelFilter) -> Config {
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

    pub fn get_html_content(filename: &str) -> String {
        let file_path = format!("tests/{}", filename);
        let mut file = File::open(file_path).expect("file not found");

        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("unable to read sample file");

        WINDOWS_1251.decode(&data, DecoderTrap::Strict)
            .expect("unable to get sample html data")
    }
}