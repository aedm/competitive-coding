mod advent_of_code_2020;
mod euler;
mod ioi;
mod utils;
mod others;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let solution = others::pin_test::solve();
    let elapsed = now.elapsed().as_micros();
    println!("Solution:\n{:?}", solution);
    println!("Runtime: {} sec", elapsed as f64 / 1000000.0);
}
