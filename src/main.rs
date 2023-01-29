use std::time::Instant;
use aoc_y2022::run_all;

fn main() {
    let start = Instant::now();

    run_all();

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}