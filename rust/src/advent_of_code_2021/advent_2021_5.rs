use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::{max, min};

fn order_swap(x: i64, y: i64) -> (i64, i64) {
    (min(x, y), max(x, y))
}

pub fn solve(diagonals: bool) -> i64 {
    let lines = read_lines("advent_2021/5.txt");
    let rows: Vec<_> = lines
        .iter()
        .map(|x| {
            x.replace(" -> ", ",")
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    let mut e = vec![];
    for r in rows {
        if r[0] == r[2] || r[1] == r[3] {
            let (x1, x2) = order_swap(r[0], r[2]);
            let (y1, y2) = order_swap(r[1], r[3]);
            (y1..=y2).cartesian_product(x1..=x2).for_each(|c| e.push(c));
        } else if diagonals {
            let (a, b) = order_swap(r[0], r[2]);
            let dx = (r[2] > r[0]) as i64 * 2 - 1;
            let dy = (r[3] > r[1]) as i64 * 2 - 1;
            (0..=(b - a)).for_each(|n| e.push((r[1] + n * dy, r[0] + n * dx)));
        }
    }
    e.sort();
    e.iter()
        .group_by(|&&e| e)
        .into_iter()
        .map(|(key, items)| (items.count() >= 2).then(|| Some(key)))
        .flatten()
        .count() as i64
}

pub fn solve_1() -> i64 {
    solve(false)
}

pub fn solve_2() -> i64 {
    solve(true)
}
