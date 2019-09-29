extern crate clap;

use clap::{App, AppSettings, Arg};
use fast_count::config::Config;
use fast_count::counter::Counter;
use fast_count::runner::run;

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
            Arg::with_name("words")
                .short("w")
                .long("words")
                .required(false)
                .help("Prints the word counts"),
        )
        .arg(
            Arg::with_name("FILE")
                .help("Sets the input files to use")
                .required(true)
                .multiple(true)
                .index(1),
        )
        .get_matches();

    let filenames: Vec<&str> = matches.values_of("FILE").unwrap().collect();
    let config = Config::new(&matches);

    let counters: Vec<Counter> = filenames
        .iter()
        .enumerate()
        .map(|(i, filename)| Counter::new(i, filename, &config))
        .collect();

    run(counters)
}
