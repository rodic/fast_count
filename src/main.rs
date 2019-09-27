extern crate clap;

use clap::{App, AppSettings, Arg};
use fast_count::config::Config;
use fast_count::counter::Counter;

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
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let config = Config::new(&matches);
    let mut counter = Counter::new(&config);

    match counter.count() {
        Err(e) => panic!("Failed to count: {}", e),
        Ok(counter) => println!("{:?}", counter),
    };
}
