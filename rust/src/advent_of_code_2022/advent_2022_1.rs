use crate::utils::read_lines;
use itertools::Itertools;

pub fn solve_1() -> i64 {
    let lines = read_lines("advent_2022/1.txt");
    lines
        .split(|l| l.is_empty())
        .map(|v| v.iter().map(|s| s.parse::<i64>().unwrap()).sum())
        .max()
        .unwrap()
}

pub fn solve_2() -> i64 {
    let lines = read_lines("advent_2022/1.txt");
    lines
        .split(|l| l.is_empty())
        .map(|v| v.iter().map(|s| s.parse::<i64>().unwrap()).sum::<i64>())
        .sorted()
        .rev()
        .take(3)
        .sum::<i64>()
}
