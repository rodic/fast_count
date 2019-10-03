extern crate clap;

use clap::{App, AppSettings, Arg};
use fast_count::config::Config;
use fast_count::counter::Counter;
use fast_count::filename_parser;
use fast_count::runner;

fn main() {
    let matches = App::new("Fast count")
        .version("1.0")
        .author("Aleksandar Rodić. <arodic@arodic.tech>")
        .about(
"Print newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified.

A word is a non-zero-length sequence of characters delimited by white space.

With no FILE, or when FILE is -, read standard input.

The options below may be used to select which counts are printed, always in the following order: newline, word, character, byte, maximum line length."
        )
        .setting(AppSettings::AllowMissingPositional)
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .required(false)
                .help("Prints the byte counts"),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .required(false)
                .help("Prints the newline counts"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .required(false)
                .help("Prints the word counts"),
        )
        .arg(
            Arg::with_name("FILE")
                .help("Sets the input files to use")
                .required(false)
                .multiple(true)
                .index(1),
        )
        .get_matches();

    let filenames = filename_parser::parse(matches.values_of("FILE"));
    let config = Config::new(&matches);

    let counters: Vec<Counter> = filenames
        .iter()
        .enumerate()
        .map(|(i, filename)| Counter::new(i, filename, &config))
        .collect();

    runner::run(counters)
}
