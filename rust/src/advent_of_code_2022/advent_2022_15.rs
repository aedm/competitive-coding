use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::max;

fn read_input() -> Vec<Vec<i64>> {
    read_lines("advent_2022/15.txt")
        .into_iter()
        .map(|s| s.split(&['=', ',', ':']).map(|w| w.parse::<i64>()).flatten().collect_vec())
        .collect_vec()
}

fn cut_at(y: i64, n: &Vec<Vec<i64>>) -> Vec<(i64, i64)> {
    n.iter()
        .map(|v| {
            let (sx, sy, bx, by) = (v[0], v[1], v[2], v[3]);
            let d = (sx - bx).abs() + (sy - by).abs();
            let r = d - (sy - y).abs();
            return if r > 0 { Some((sx - r, sx + r)) } else { None };
        })
        .flatten()
        .sorted()
        .collect_vec()
}

pub fn solve_1() -> i64 {
    cut_at(2_000_000, &read_input())
        .into_iter()
        .fold((i64::MIN, 0), |(last, sum), (x1, x2)| {
            if x1 <= last {
                let new_last = max(last, x2);
                (new_last, sum + new_last - last)
            } else {
                (x2, sum + x2 - x1)
            }
        })
        .1
}

pub fn solve_2() -> i64 {
    let n = read_input();
    for y in 0..=4_000_000 {
        let mut last = -1;
        for p in cut_at(y, &n) {
            if p.0 > last + 1 {
                return (last + 1) * 4_000_000 + y;
            }
            last = max(last, p.1);
        }
    }
    panic!()
}
