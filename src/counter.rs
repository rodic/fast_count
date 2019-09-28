use super::config::Config;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Lines};

#[derive(Debug)]
pub struct Counter<'a> {
    filename: &'a str,
    number_of_lines: Option<i32>,
    number_of_words: Option<i32>,
}

impl <'a>Counter<'a> {
    pub fn new(filename: &'a str, config: &Config) -> Counter<'a> {
        let number_of_lines = Counter::flag_to_option(config.should_count_lines);
        let number_of_words = Counter::flag_to_option(config.should_count_words);

        Counter {
            filename,
            number_of_lines,
            number_of_words,
        }
    }

    pub fn count(&mut self) -> Result<&Counter, io::Error> {
        for line in Counter::get_lines(&self.filename)? {
            self.number_of_lines = Counter::add(self.number_of_lines, 1);
            self.number_of_words = Counter::add(
                self.number_of_words,
                Counter::count_words_in_line(&line?)
            );
        }
        Ok(&*self)
    }

    fn count_words_in_line(line: &str) -> i32 {
        line.split_whitespace().count().try_into().unwrap()
    }

    fn flag_to_option(flag: bool) -> Option<i32> {
        if flag {
            Some(0)
        } else {
            None
        }
    }

    fn add(option: Option<i32>, increment: i32) -> Option<i32> {
        match option {
            Some(n) => Some(n + increment),
            None => None,
        }
    }

    fn get_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
        let file = File::open(filename)?;
        Ok(BufReader::new(file).lines())
    }
}
