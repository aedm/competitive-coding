use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::{max, min};

pub fn solve(diagonals: bool) -> i64 {
    let rows = read_lines("advent_2021/5.txt")
        .iter()
        .map(|x| x.replace(" -> ", ",").split(',').map(|s| s.parse::<i64>().unwrap()).collect_vec())
        .collect_vec();
    let mut fields = vec![];
    for r in rows {
        if r[0] == r[2] || r[1] == r[3] || diagonals {
            let (rx, ry) = (r[2] - r[0], r[3] - r[1]);
            let (dx, dy) = (rx.signum(), ry.signum());
            (0..=max(rx.abs(), ry.abs())).for_each(|n| fields.push((r[0] + n * dx, r[1] + n * dy)));
        }
    }
    fields.sort();
    fields.iter().group_by(|&&e| e).into_iter().map(|x| (x.1.count() >= 2) as i64).sum()
}

pub fn solve_1() -> i64 {
    solve(false)
}

pub fn solve_2() -> i64 {
    solve(true)
}
