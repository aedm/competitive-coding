use crate::utils::{neighbours4, read_lines};
use itertools::Itertools;
use regex::Regex;
use std::any::Any;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn read() -> (i64, i64, i64, i64) {
    let mut lines = read_lines("advent_2021/17.txt");
    let pattern: Regex =
        Regex::new(r"^target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)$").unwrap();
    let caps = pattern.captures(&lines[0]).unwrap();
    let c = caps.iter().skip(1).map(|c| c.unwrap().as_str().parse::<i64>().unwrap()).collect_vec();
    (c[0], c[1], c[2], c[3])
}

pub fn solve_1() -> i64 {
    let (_, _, y1, _) = read();
    -y1 * (-y1 - 1) / 2
}

pub fn solve_2() -> usize {
    let (x1, x2, y1, y2) = read();
    let ym = -min(y1, y2);
    ((1..=x2).cartesian_product(-ym..=ym))
        .filter(|&(xf, yf)| {
            let (mut x, mut y, mut vx, mut vy) = (0, 0, xf, yf);
            while y >= -ym {
                if x == x.clamp(x1, x2) && y == y.clamp(y1, y2) {
                    return true;
                }
                x += vx;
                y += vy;
                vx = max(vx - 1, 0);
                vy -= 1;
            }
            false
        })
        .count()
}
