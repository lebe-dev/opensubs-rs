#[cfg(test)]
pub mod test_utils {
    use std::fs::File;
    use std::io::Read;

    use encoding::{DecoderTrap, Encoding};
    use encoding::all::WINDOWS_1251;

    pub fn get_html_content(filename: &str) -> String {
        let file_path = format!("tests/{}", filename);
        let mut file = File::open(file_path).expect("file not found");

        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("unable to read sample file");

        WINDOWS_1251.decode(&data, DecoderTrap::Strict)
            .expect("unable to get sample html data")
    }
}