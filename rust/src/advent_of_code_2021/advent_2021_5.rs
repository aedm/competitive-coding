use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::{max, min};

pub fn solve(diagonals: bool) -> i64 {
    let rows = read_lines("advent_2021/5.txt")
        .iter()
        .map(|x| x.replace(" -> ", ",").split(',').map(|s| s.parse::<i64>().unwrap()).collect_vec())
        .collect_vec();
    let mut fields = vec![];
    for r in rows.iter().filter(|r| r[0] == r[2] || r[1] == r[3] || diagonals) {
        let (dx, dy) = ((r[2] - r[0]).signum(), (r[3] - r[1]).signum());
        let length = max((r[2] - r[0]).abs(), (r[3] - r[1]).abs());
        (0..=length).for_each(|n| fields.push((r[0] + n * dx, r[1] + n * dy)));
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
