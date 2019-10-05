use super::config::Config;
use super::input_reader::InputReader;
use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::io::{self, BufRead};

pub struct Counter {
    id: usize,
    pub filename: String,
    number_of_lines: Option<u32>,
    number_of_words: Option<u32>,
    number_of_chars: Option<u32>,
    number_of_bytes: Option<u32>,
    max_line_length: Option<u32>,
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
        result.push_str(&Counter::format_option(self.number_of_chars));
        result.push_str(&Counter::format_option(self.number_of_bytes));
        result.push_str(&Counter::format_option(self.max_line_length));
        result.push_str(&self.filename);

        write!(f, "{}", result)
    }
}

impl Counter {
    pub fn new(id: usize, filename: &str, config: &Config) -> Counter {
        let number_of_lines = Counter::flag_to_option(config.count_lines);
        let number_of_words = Counter::flag_to_option(config.count_words);
        let number_of_chars = Counter::flag_to_option(config.count_chars);
        let number_of_bytes = Counter::flag_to_option(config.count_bytes);
        let max_line_length = Counter::flag_to_option(config.count_max_line_length);

        Counter {
            id,
            filename: String::from(filename),
            number_of_lines,
            number_of_words,
            number_of_chars,
            number_of_bytes,
            max_line_length,
        }
    }

    pub fn null_counter() -> Counter {
        Counter {
            id: 0,
            filename: String::new(),
            number_of_lines: None,
            number_of_words: None,
            number_of_chars: None,
            number_of_bytes: None,
            max_line_length: None,
        }
    }

    pub fn count(&mut self) -> Result<&Counter, io::Error> {
        let mut line = String::new();

        let input_reader = InputReader::new(&self.filename);
        let mut buffer_reader = input_reader.get_buffer_reader();

        loop {
            match buffer_reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(number_of_bytes) => {
                    if let Some(n) = self.number_of_lines {
                        self.number_of_lines = Some(n + 1);
                    }
                    if let Some(n) = self.number_of_words {
                        self.number_of_words = Some(n + Counter::count_words_in_line(&line));
                    }
                    if let Some(n) = self.number_of_chars {
                        self.number_of_chars = Some(n + Counter::count_chars_in_line(&line))
                    }
                    if let Some(n) = self.number_of_bytes {
                        self.number_of_bytes = Some(n + number_of_bytes as u32)
                    }
                    if let Some(n) = self.max_line_length {
                        self.max_line_length = Some(n.max(line.trim_end().len() as u32));
                    }
                }
                Err(e) => panic!(e),
            }
            line.clear();
        }
        Ok(&*self)
    }

    fn count_words_in_line(line: &str) -> u32 {
        line.split_whitespace().count() as u32
    }

    fn count_chars_in_line(line: &str) -> u32 {
        line.chars().count() as u32
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
            format!("{:>7} ", n)
        } else {
            String::from("")
        }
    }

    pub fn add(&self, other: &Counter) -> Counter {
        let number_of_lines = Counter::sum_options(self.number_of_lines, other.number_of_lines);
        let number_of_words = Counter::sum_options(self.number_of_words, other.number_of_words);
        let number_of_chars = Counter::sum_options(self.number_of_chars, other.number_of_chars);
        let number_of_bytes = Counter::sum_options(self.number_of_bytes, other.number_of_bytes);
        let max_line_length = Counter::max_options(self.max_line_length, other.max_line_length);

        Counter {
            id: 0,
            filename: String::from("total"),
            number_of_lines,
            number_of_words,
            number_of_chars,
            number_of_bytes,
            max_line_length,
        }
    }

    fn sum_options(x: Option<u32>, y: Option<u32>) -> Option<u32> {
        match (x, y) {
            (Some(x), Some(y)) => Some(x + y),
            (None, y) => y,
            (x, None) => x,
        }
    }

    fn max_options(x: Option<u32>, y: Option<u32>) -> Option<u32> {
        match (x, y) {
            (Some(x), Some(y)) => Some(x.max(y)),
            (None, y) => y,
            (x, None) => x,
        }
    }
}
