use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::min;

fn solve(smudge_count: usize) -> usize {
    read_lines("advent_2023/13.txt")
        .split(|l| l.is_empty())
        .map(|b| {
            let b = b.iter().map(|l| l.chars().collect_vec()).collect_vec();
            let (w, h) = (b[0].len(), b.len());
            let vertical = (1..w).find(|&x| {
                Itertools::cartesian_product(0..min(x, w - x), 0..h)
                    .filter(|&(i, j)| b[j][x - i - 1] != b[j][x + i])
                    .count()
                    == smudge_count
            });
            let horizontal = (1..h).find(|&y| {
                Itertools::cartesian_product(0..min(y, h - y), 0..w)
                    .filter(|&(i, j)| b[y - i - 1][j] != b[y + i][j])
                    .count()
                    == smudge_count
            });
            horizontal.unwrap_or(0) * 100 + vertical.unwrap_or(0)
        })
        .sum()
}

pub fn solve_1() -> usize {
    solve(0)
}

pub fn solve_2() -> usize {
    solve(1)
}
