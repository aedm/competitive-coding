use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::{max, min};

const SIZE: usize = 2_000;
const OFFSET: usize = 500;

fn read_map() -> (Vec<[bool; SIZE]>, usize) {
    let mut m = vec![[false; SIZE]; SIZE];
    let mut floor = 0;
    for line in read_lines("advent_2022/14.txt").into_iter() {
        let l = line
            .split(" -> ")
            .map(|s| s.split(",").map(|n| n.parse::<usize>().unwrap()).collect_vec())
            .collect_vec();
        floor = max(floor, l.iter().map(|v| v[1]).max().unwrap());
        for v in l.windows(2) {
            let (minx, miny) = (min(v[0][0], v[1][0]), min(v[0][1], v[1][1]));
            let (maxx, maxy) = (max(v[0][0], v[1][0]), max(v[0][1], v[1][1]));
            (minx..=maxx).cartesian_product(miny..=maxy).for_each(|(x, y)| m[y][x + OFFSET] = true);
        }
    }
    (m, floor)
}

pub fn solve_1() -> usize {
    let (mut m, _) = read_map();
    (0..)
        .take_while(|_| {
            let (mut x, mut y) = (500 + OFFSET as isize, 0usize);
            while let Some(&d) = [0, -1, 1].iter().find(|&&d| !m[y + 1][(x + d) as usize]) {
                (x, y) = (x + d, y + 1);
                if y + 1 >= SIZE {
                    return false;
                }
            }
            m[y][x as usize] = true;
            true
        })
        .count()
}

pub fn solve_2() -> usize {
    let (mut m, floor) = read_map();
    (0..SIZE).for_each(|x| m[floor + 2][x] = true);
    (0..)
        .take_while(|_| {
            let (mut x, mut y) = (500 + OFFSET as isize, 0usize);
            while let Some(&d) = [0, -1, 1].iter().find(|&&d| !m[y + 1][(x + d) as usize]) {
                (x, y) = (x + d, y + 1);
            }
            m[y][x as usize] = true;
            y != 0
        })
        .count()
        + 1
}
