use crate::utils::read_lines;
use itertools::Itertools;

pub fn solve_1() -> u32 {
    read_lines("advent_2023/1.txt")
        .iter()
        .map(|l| {
            let v = l.chars().filter_map(|c| c.to_digit(10)).collect_vec();
            v[0] * 10 + v[v.len() - 1]
        })
        .sum()
}

pub fn solve_2() -> usize {
    let digits = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];

    read_lines("advent_2023/1.txt")
        .iter()
        .map(|l| {
            let v = (0..l.len())
                .filter_map(|i| {
                    digits
                        .iter()
                        .enumerate()
                        .find(|(o, d)| l[i..].starts_with(**d))
                        .map(|(o, _)| o % 10)
                })
                .collect_vec();
            v[0] * 10 + v[v.len() - 1]
        })
        .sum()
}
