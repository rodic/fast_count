use super::config::Config;
use super::input_reader::InputReader;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct Counter {
    id: usize,
    pub filename: String,
    number_of_lines: Option<u32>,
    number_of_words: Option<u32>,
}

impl Ord for Counter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Counter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl PartialEq for Counter {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Counter {}

impl Counter {
    pub fn new(id: usize, filename: &str, config: &Config) -> Counter {
        let number_of_lines = Counter::flag_to_option(config.should_count_lines);
        let number_of_words = Counter::flag_to_option(config.should_count_words);

        Counter {
            id,
            filename: String::from(filename),
            number_of_lines,
            number_of_words,
        }
    }

    pub fn count(&mut self) -> Result<&Counter, io::Error> {
        for line in InputReader::new(&self.filename).read().lines() {
            self.number_of_lines = Counter::add(self.number_of_lines, 1);
            self.number_of_words =
                Counter::add(self.number_of_words, Counter::count_words_in_line(&line?));
        }
        Ok(&*self)
    }

    fn count_words_in_line(line: &str) -> u32 {
        line.split_whitespace().count().try_into().unwrap()
    }

    fn flag_to_option(flag: bool) -> Option<u32> {
        if flag {
            Some(0)
        } else {
            None
        }
    }

    fn add(option: Option<u32>, increment: u32) -> Option<u32> {
        match option {
            Some(n) => Some(n + increment),
            None => None,
        }
    }
}
