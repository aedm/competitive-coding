use crate::utils::read_lines;
use itertools::Itertools;
use regex::Regex;

pub fn solve_1() -> i64 {
    let pattern: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    read_lines("advent_2022/4.txt")
        .iter()
        .filter(|l| {
            let caps = pattern.captures(l).unwrap();
            let n = (1usize..=4)
                .map(|i| caps.get(i).unwrap().as_str().parse::<usize>().unwrap())
                .collect_vec();
            (n[0] >= n[2] && n[1] <= n[3]) || (n[0] <= n[2] && n[1] >= n[3])
        })
        .count() as i64
}

pub fn solve_2() -> i64 {
    let pattern: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    read_lines("advent_2022/4.txt")
        .iter()
        .filter(|l| {
            let caps = pattern.captures(l).unwrap();
            let n = (1usize..=4)
                .map(|i| caps.get(i).unwrap().as_str().parse::<usize>().unwrap())
                .collect_vec();
            (n[0] >= n[2] && n[0] <= n[3]) || (n[2] >= n[0] && n[2] <= n[1])
        })
        .count() as i64
}
