use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::min;

pub fn solve_1() -> usize {
    read_lines("advent_2023/13.txt")
        .split(|l| l.is_empty())
        .map(|b| {
            let b = b.iter().map(|l| l.chars().collect_vec()).collect_vec();
            let (w, h) = (b[0].len(), b.len());
            let vertical = (1..w)
                .find(|&x| {
                    let lc = min(x, w - x);
                    (0..lc).cartesian_product(0..h).all(|(i, j)| b[j][x - i - 1] == b[j][x + i])
                })
                .unwrap_or(0);
            let horizontal = (1..h)
                .find(|&y| {
                    let lc = min(y, h - y);
                    (0..lc).cartesian_product(0..w).all(|(i, j)| b[y - i - 1][j] == b[y + i][j])
                })
                .unwrap_or(0);
            horizontal * 100 + vertical
        })
        .sum::<usize>()
}

pub fn solve_2() -> usize {
    read_lines("advent_2023/13.txt")
        .split(|l| l.is_empty())
        .into_iter()
        .map(|b| {
            let b = b.iter().map(|l| l.chars().collect_vec()).collect_vec();
            let (w, h) = (b[0].len(), b.len());
            let vertical = (1..w)
                .find(|&x| {
                    let lc = min(x, w - x);
                    (0..lc)
                        .cartesian_product(0..h)
                        .filter(|&(i, j)| b[j][x - i - 1] != b[j][x + i])
                        .count()
                        == 1
                })
                .unwrap_or(0);
            let horizontal = (1..h)
                .find(|&y| {
                    let lc = min(y, h - y);
                    (0..lc)
                        .cartesian_product(0..w)
                        .filter(|&(i, j)| b[y - i - 1][j] != b[y + i][j])
                        .count()
                        == 1
                })
                .unwrap_or(0);
            horizontal * 100 + vertical
        })
        .sum::<usize>()
}
