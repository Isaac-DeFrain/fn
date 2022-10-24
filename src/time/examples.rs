// time examples

use std::thread;
use std::time::{Duration, Instant};

pub fn main() {
    let start = Instant::now();
    println!("A: {:?}", start);
    thread::sleep(Duration::from_millis(1));
    println!("B: {:?}", start.elapsed());
}
