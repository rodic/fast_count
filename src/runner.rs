use super::counter::Counter;
use std::sync::mpsc;
use std::thread;

pub fn run(counters: Vec<Counter>) {
    let (tx, rx) = mpsc::channel();
    let number_of_workers = counters.len();

    for mut counter in counters {
        let tx = tx.clone();
        thread::spawn(move || {
            &counter.count();
            tx.send(counter)
        });
    }

    let mut counters: Vec<Counter> = vec![];

    for _ in 0..number_of_workers {
        match rx.recv() {
            Ok(c) => counters.push(c),
            Err(e) => panic!("Failed to perform count {}", e),
        };
    }

    counters.sort();

    for counter in counters {
        println!("{:?}", counter);
    }
}
