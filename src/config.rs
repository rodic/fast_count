extern crate clap;

use clap::ArgMatches;

#[derive(Debug)]
pub struct Config {
    pub should_count_lines: bool,
    pub should_count_words: bool,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Config {
        let should_count_lines = Config::parse_flag(matches, "lines");
        let should_count_words = Config::parse_flag(matches, "words");

        Config {
            should_count_lines,
            should_count_words,
        }
    }

    fn parse_flag(matches: &ArgMatches, flag: &str) -> bool {
        match matches.occurrences_of(flag) {
            1 => true,
            _ => false,
        }
    }
}
