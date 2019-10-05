use clap::ArgMatches;

pub struct Config {
    pub count_lines: bool,
    pub count_words: bool,
    pub count_chars: bool,
    pub count_bytes: bool,
    pub count_max_line_length: bool,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Config {
        let count_lines = Config::parse_flag(matches, "lines");
        let count_words = Config::parse_flag(matches, "words");
        let count_chars = Config::parse_flag(matches, "chars");
        let count_bytes = Config::parse_flag(matches, "bytes");
        let count_max_line_length = Config::parse_flag(matches, "max-line-length");

        let (count_lines, count_words, count_bytes) = Config::with_defaults(
            count_lines,
            count_words,
            count_chars,
            count_bytes,
            count_max_line_length,
        );

        Config {
            count_lines,
            count_words,
            count_chars,
            count_bytes,
            count_max_line_length,
        }
    }

    fn parse_flag(matches: &ArgMatches, flag: &str) -> bool {
        match matches.occurrences_of(flag) {
            1 => true,
            _ => false,
        }
    }

    fn with_defaults(
        lines: bool,
        words: bool,
        chars: bool,
        bytes: bool,
        max_line: bool,
    ) -> (bool, bool, bool) {
        if let (false, false, false, false, false) = (lines, words, chars, bytes, max_line) {
            (true, true, true)
        } else {
            (lines, words, bytes)
        }
    }
}
