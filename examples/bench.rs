#[macro_use]
extern crate lazy_static;

use std::thread;
use std::time;

const N_THREADS: usize = 32;
const N_ROUNDS: usize = 100_000_000;

lazy_static! {
    static ref XS: Vec<String> = vec!["Spica".to_string(), "Hoyten".to_string()];
}

fn main() {
    let start = time::Instant::now();
    let threads = (0..N_THREADS)
        .map(|_| thread::spawn(move || thread_main()))
        .collect::<Vec<_>>();
    for thread in threads {
        thread.join().unwrap();
    }
    let elapsed = start.elapsed();
    println!("{:?}", elapsed)
}

fn thread_main() {
    for _ in 0..N_ROUNDS {
        let len = XS.len();
        assert_eq!(len, 2)
    }
}
