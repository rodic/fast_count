use super::config::Config;
use super::input_reader::InputReader;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::fmt::{self, Display, Formatter};
use std::io::{self, BufRead};

pub struct Counter {
    id: usize,
    pub filename: String,
    number_of_bytes: Option<u32>,
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

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&Counter::format_option(self.number_of_lines));
        result.push_str(&Counter::format_option(self.number_of_words));
        result.push_str(&Counter::format_option(self.number_of_bytes));
        result.push_str(&self.filename);

        write!(f, "{}", result)
    }
}

impl Counter {
    pub fn new(id: usize, filename: &str, config: &Config) -> Counter {
        let number_of_bytes = Counter::flag_to_option(config.count_bytes);
        let number_of_lines = Counter::flag_to_option(config.count_lines);
        let number_of_words = Counter::flag_to_option(config.count_words);

        Counter {
            id,
            filename: String::from(filename),
            number_of_bytes,
            number_of_lines,
            number_of_words,
        }
    }

    pub fn count(&mut self) -> Result<&Counter, io::Error> {
        for line in InputReader::new(&self.filename).read().lines() {
            let line = line?;
            if let Some(n) = self.number_of_bytes {
                self.number_of_bytes = Some(n + Counter::count_bytes_in_line(&line))
            }
            if let Some(n) = self.number_of_lines {
                self.number_of_lines = Some(n + 1);
            }

            if let Some(n) = self.number_of_words {
                self.number_of_words = Some(n + Counter::count_words_in_line(&line));
            }
        }
        Ok(&*self)
    }

    fn count_words_in_line(line: &str) -> u32 {
        line.split_whitespace().count().try_into().unwrap()
    }

    fn count_bytes_in_line(line: &str) -> u32 {
        line.as_bytes().len().try_into().unwrap()
    }

    fn flag_to_option(flag: bool) -> Option<u32> {
        if flag {
            Some(0)
        } else {
            None
        }
    }

    fn format_option(n: Option<u32>) -> String {
        if let Some(n) = n {
            format!("{:7} ", n)
        } else {
            String::from("")
        }
    }
}
