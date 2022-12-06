use crate::utils::read_lines;
use itertools::Itertools;

pub fn solve_1() -> i64 {
    read_lines("advent_2022/2.txt")
        .iter()
        .map(|l| {
            let c = l.as_bytes();
            let (a, b) = (c[0] as i64 - 'A' as i64, c[2] as i64 - 'X' as i64);
            ((b - a + 4) % 3) * 3 + b + 1
        })
        .sum()
}

pub fn solve_2() -> i64 {
    read_lines("advent_2022/2.txt")
        .iter()
        .map(|l| {
            let c = l.as_bytes();
            let (a, b) = (c[0] as i64 - 'A' as i64, c[2] as i64 - 'X' as i64);
            b * 3 + (a + 2 + b) % 3 + 1
        })
        .sum()
}
