use crate::utils::read_lines;
use itertools::Itertools;

pub fn card_matches() -> Vec<usize> {
    read_lines("advent_2023/4.txt")
        .iter()
        .map(|l| {
            let p = l.split(": ").nth(1).unwrap().split('|');
            let nums = p
                .map(|s| s.split(' ').filter_map(|s| s.parse::<usize>().ok()).collect_vec())
                .collect_vec();
            nums[0].iter().filter(|v| nums[1].contains(v)).count()
        })
        .collect_vec()
}

pub fn solve_1() -> usize {
    card_matches().iter().map(|m| 1 << m >> 1).sum()
}

pub fn solve_2() -> usize {
    let m = card_matches();
    let mut c = vec![1; m.len()];
    for i in 0..m.len() {
        for p in 1..=m[i] {
            c[i + p] += c[i];
        }
    }
    c.iter().sum()
}
