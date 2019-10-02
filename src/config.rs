use clap::ArgMatches;

pub struct Config {
    pub count_lines: bool,
    pub count_words: bool,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Config {
        let count_lines = Config::parse_flag(matches, "lines");
        let count_words = Config::parse_flag(matches, "words");

        let (count_lines, count_words) = Config::with_defaults(count_lines, count_words);

        Config {
            count_lines,
            count_words,
        }
    }

    fn parse_flag(matches: &ArgMatches, flag: &str) -> bool {
        match matches.occurrences_of(flag) {
            1 => true,
            _ => false,
        }
    }

    fn with_defaults(count_lines: bool, count_words: bool) -> (bool, bool) {
        if let (false, false) = (count_lines, count_words) {
            (true, true)
        } else {
            (count_lines, count_words)
        }
    }
}
