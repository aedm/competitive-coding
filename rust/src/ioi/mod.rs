mod ioi_2019_1;
mod task_runner;

use std::time::Instant;

pub fn run() {
    let now = Instant::now();
    // let solution = ioi_2019_1::solve();
    let _ = task_runner::run("./resources/ioi_2019_1", ioi_2019_1::solve);
    let elapsed = now.elapsed().as_micros();
    println!("Runtime: {} sec", elapsed as f64 / 1000000.0);
}
