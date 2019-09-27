use std::fs::File;
use std::io::{self, prelude::*, BufReader, Lines};
use super::config::{Config};

#[derive(Debug)]
pub struct Counter<'a> {
    config: &'a Config<'a>,
    number_of_lines: Option<u32>,
    number_of_words: Option<u32>,
}

impl<'a> Counter<'a> {
    pub fn new(config: &'a Config) -> Counter<'a> {
        let number_of_lines = Counter::flag_to_option(config.should_count_lines);
        let number_of_words = Counter::flag_to_option(config.should_count_words);

        Counter {
            config,
            number_of_lines,
            number_of_words,
        }
    }

    pub fn count(&mut self) -> Result<(), io::Error> {
        for line in Counter::get_lines(self.config.filename)? {
            self.number_of_lines = Counter::add(self.number_of_lines, 1);
            self.number_of_words = Counter::add(
                self.number_of_words,
                line?.split_whitespace().count() as u32
            );
        }
        Ok(())
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

    fn get_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
        let file = File::open(filename)?;
        Ok(BufReader::new(file).lines())
    }
}
