use crate::utils::read_lines_split;
use itertools::{iterate, Itertools};

fn read_input() -> impl Iterator<Item = Vec<Vec<i64>>> {
    read_lines_split("advent_2023/9.txt", &[' ']).into_iter().map(|l| {
        let n = l.iter().map(|s| s.parse().unwrap()).collect_vec();
        iterate(n, |v| (1..v.len()).map(|i| v[i] - v[i - 1]).collect_vec())
            .take_while(|v| v.len() > 1)
            .collect_vec()
    })
}

pub fn solve_1() -> i64 {
    read_input().map(|n| n.iter().map(|n| n[n.len() - 1]).sum::<i64>()).sum()
}

pub fn solve_2() -> i64 {
    read_input().map(|n| (1..n.len()).fold(0, |a, i| n[n.len() - 1 - i][0] - a)).sum()
}
