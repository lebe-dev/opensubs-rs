#[cfg(test)]
mod strip_tests {
    use crate::strip::strip::strip_html_tags;

    #[test]
    fn html_tags_should_be_remove_from_string() {
        assert_eq!(
            strip_html_tags("Some text <span class=\"brackets-pair3000\">with</span> tags."),
            "Some text with tags."
        );
        assert_eq!(
            strip_html_tags("Hi<br>How about multiple tags?<span>*</span>"),
            "HiHow about multiple tags?*"
        );
        assert_eq!(
            strip_html_tags("How <span style=\"font-size: 12px\">about</span> html-attributes?"),
            "How about html-attributes?"
        );
        assert_eq!(
            strip_html_tags("How&nbsp;about&nbsp;special&nbsp;combinations?"),
            "How about special combinations?"
        );
        assert_eq!(
            strip_html_tags("12.45&nbsp;GB ↓"),
            "12.45 GB ↓"
        );
        assert_eq!(
            strip_html_tags("12.45\u{a0}GB ↓"),
            "12.45 GB ↓"
        );

        assert_eq!(
            strip_html_tags("<a data-topic_id=\"2510633\" class=\"med tLink ts-text hl-tags bold\" href=\"https://rutracker.org/forum/viewtopic.php?t=2510633\">Терминатор: Хроники Сары Коннор / Битва за будущее / Terminator: The Sarah Connor Chronicles / Сезон: 2 / Серии: 1-22 <span class=\"brackets-pair\">(22)</span> <span class=\"brackets-pair\">[2008, США, фантастика, боевик, драма, BDRemux 1080p]</span> MVO <span class=\"brackets-pair\">(DD 5.<wbr>1 LostFilm)</span></a>"),
            "Терминатор: Хроники Сары Коннор / Битва за будущее / Terminator: The Sarah Connor Chronicles / Сезон: 2 / Серии: 1-22 (22) [2008, США, фантастика, боевик, драма, BDRemux 1080p] MVO (DD 5.1 LostFilm)"
        );
    }
}
