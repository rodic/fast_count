extern crate clap;
use clap::{App, AppSettings, Arg, ArgMatches};
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Lines};

#[derive(Debug)]
struct Config<'a> {
    filename: &'a str,
    should_count_lines: bool,
}

#[derive(Debug)]
struct CountResult {
    number_of_lines: Option<u32>,
}

fn main() {
    let matches = App::new("Fast count")
        .version("1.0")
        .author("Aleksandar Rodić. <arodic@arodic.tech>")
        .about("Prints newline, word, and byte counts for each file")
        .setting(AppSettings::AllowMissingPositional)
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .required(false)
                .help("Prints the newline counts"),
        )
        .arg(
            Arg::with_name("FILE")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let config = get_config(&matches);
    let result = count(&config);

    println!("{:?}", config);
    println!("{:?}", result);
}

fn get_config<'a>(matches: &'a ArgMatches) -> Config<'a> {
    let filename = matches.value_of("FILE").unwrap();
    let should_count_lines = match matches.occurrences_of("lines") {
        1 => true,
        _ => false,
    };

    Config {
        filename,
        should_count_lines,
    }
}

fn count(config: &Config) -> Result<CountResult, io::Error> {
    let mut result = initialize_result(&config);

    for _ in get_lines(config.filename)? {
        result.number_of_lines = if let Some(n) = result.number_of_lines {
            Some(n + 1)
        } else {
            None
        };
    }
    Ok(result)
}

fn initialize_result(config: &Config) -> CountResult {
    let number_of_lines = if config.should_count_lines {
        Some(0)
    } else {
        None
    };

    CountResult { number_of_lines }
}

fn get_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
