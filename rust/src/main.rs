mod advent_of_code_2020;
mod euler;
mod ioi;
mod others;
mod utils;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let solution = others::fizzbuzz::solve();
    let elapsed = now.elapsed().as_micros();
    println!("Solution:\n{:?}", solution);
    println!("Runtime: {} sec", elapsed as f64 / 1000000.0);
}
