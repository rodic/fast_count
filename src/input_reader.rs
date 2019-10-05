use super::constants;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Stdin};

pub enum InputReader {
    FileReader(String),
    StdinReader(Stdin),
}

impl InputReader {
    pub fn new(filename: &str) -> InputReader {
        if filename == constants::STDIN {
            InputReader::StdinReader(stdin())
        } else {
            InputReader::FileReader(String::from(filename))
        }
    }

    pub fn get_buffer_reader<'a>(&'a self) -> Box<dyn BufRead + 'a> {
        match self {
            InputReader::StdinReader(stdin) => Box::new(stdin.lock()),
            InputReader::FileReader(filename) => {
                let file =
                    File::open(filename).expect(&format!("failed to open file: {}", filename));
                Box::new(BufReader::new(file))
            }
        }
    }
}
