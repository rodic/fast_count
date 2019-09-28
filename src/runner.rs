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

    for _ in 0..number_of_workers {
        match rx.recv() {
            Ok(c) => println!("{:?}", c),
            Err(e) => panic!("Failed to perform count {}", e),
        };
    }
}
