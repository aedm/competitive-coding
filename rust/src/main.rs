mod advent_of_code_2020;
mod advent_of_code_2021;
mod euler;
mod hackerrank;
mod ioi;
mod others;
mod qpa_2021;
mod utils;

use std::time::Instant;

#[tokio::main]
async fn main() {
    let now = Instant::now();
    // hackerrank::decibinary_numbers::main();
    // let solution = qpa_2021::qpa_main().await;
    let solution = advent_of_code_2021::advent_2021_7::solve_2();
    let elapsed = now.elapsed().as_micros();
    println!("Solution:\n{:?}", solution);
    println!("Runtime: {} sec", elapsed as f64 / 1000000.0);
}
