use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::max;

pub fn solve(diagonals: bool) -> i64 {
    let mut fields = vec![];
    for line in read_lines("advent_2021/5.txt") {
        let r = line.split(&[' ', ','][..]).filter_map(|s| s.parse::<i64>().ok()).collect_vec();
        if r[0] == r[2] || r[1] == r[3] || diagonals {
            let (rx, ry) = (r[2] - r[0], r[3] - r[1]);
            let (dx, dy) = (rx.signum(), ry.signum());
            (0..=max(rx.abs(), ry.abs())).for_each(|n| fields.push((r[0] + n * dx, r[1] + n * dy)));
        }
    }
    fields.iter().sorted().group_by(|&&e| e).into_iter().map(|(_, v)| (v.count() > 1) as i64).sum()
}

pub fn solve_1() -> i64 {
    solve(false)
}

pub fn solve_2() -> i64 {
    solve(true)
}
