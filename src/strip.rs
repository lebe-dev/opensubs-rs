pub mod strip {
    use regex::Regex;

    const OPEN_TAG_PATTERN: &str = "(<\\s*\\w[^>]*>)";
    const CLOSE_TAG_PATTERN: &str = "</[\\w+]+>";
    const SPACE_CHARACTER_CODE: &str = "&nbsp;";
    const SPACE_CHARACTER_UNICODE: &str = "\u{a0}";
    const BLANK_VALUE: &str = "";
    const SPACE_CHARACTER_VALUE: &str = " ";

    pub fn strip_html_tags(text: &str) -> String {
        let open_tag_regex = Regex::new(OPEN_TAG_PATTERN).unwrap();
        let close_tag_regex = Regex::new(CLOSE_TAG_PATTERN).unwrap();

        let removed_open_tags = open_tag_regex.replace_all(text, BLANK_VALUE).to_string();
        let removed_close_tags = close_tag_regex.replace_all(
            &removed_open_tags, BLANK_VALUE
        ).to_string();

        let removed_space_character_codes = removed_close_tags.replace(
            SPACE_CHARACTER_CODE, SPACE_CHARACTER_VALUE
        ).to_string();

        let removed_space_character_unicodes = removed_space_character_codes.replace(
            SPACE_CHARACTER_UNICODE, SPACE_CHARACTER_VALUE
        ).to_string();

        String::from(removed_space_character_unicodes)
    }
}