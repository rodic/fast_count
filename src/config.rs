use clap::ArgMatches;

pub struct Config {
    pub count_lines: bool,
    pub count_words: bool,
    pub count_chars: bool,
    pub count_bytes: bool,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Config {
        let count_lines = Config::parse_flag(matches, "lines");
        let count_words = Config::parse_flag(matches, "words");
        let count_chars = Config::parse_flag(matches, "chars");
        let count_bytes = Config::parse_flag(matches, "bytes");

        let (count_bytes, count_lines, count_words) =
            Config::with_defaults(count_bytes, count_lines, count_words);

        Config {
            count_lines,
            count_words,
            count_chars,
            count_bytes,
        }
    }

    fn parse_flag(matches: &ArgMatches, flag: &str) -> bool {
        match matches.occurrences_of(flag) {
            1 => true,
            _ => false,
        }
    }

    fn with_defaults(count_bytes: bool, count_lines: bool, count_words: bool) -> (bool, bool, bool) {
        if let (false, false, false) = (count_bytes, count_lines, count_words) {
            (true, true, true)
        } else {
            (count_bytes, count_lines, count_words)
        }
    }
}
