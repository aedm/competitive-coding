use crate::utils::read_lines;
use std::cmp::{max, min};

pub fn solve() -> String {
    let lines = read_lines("scratch.txt");
    let inputs: Vec<_> = lines
        .iter()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    solve_all(&inputs)
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn solve_all(inputs: &Vec<u64>) -> Vec<u64> {
    let max = 10_000_000_000_000_000u64;
    let mut a = vec![1, 1];
    let mut k = 2;
    let mut i = 2;
    while k <= max {
        let an = calc_k(i, &a);
        // println!("{}: {}", i, an);
        a.push(an);
        k += an;
        i += 1;
    }

    vec![1, 2, 3]
}

fn calc_k(k: u64, a: &Vec<u64>) -> u64 {
    let m = min(k / 2, 4);
    (0..=m).map(|x| a[(k / 2 - x) as usize]).sum()
}
