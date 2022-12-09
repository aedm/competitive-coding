use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve_1() -> i64 {
    let mut t = (0i64, 0i64);
    let mut h = (0, 0);
    let mut pos = HashSet::from([t]);
    for line in read_lines("advent_2022/9.txt") {
        let p = line.split(' ').collect_vec();
        let d = p[0].as_bytes()[0];
        let l = p[1].parse::<i64>().unwrap();
        let dir = match d {
            b'U' => (0, 1),
            b'R' => (1, 0),
            b'D' => (0, -1),
            b'L' => (-1, 0),
            _ => panic!(),
        };
        for i in 0..l {
            h = (h.0 + dir.0, h.1 + dir.1);
            if (h.0 - t.0).abs() > 1 || (h.1 - t.1).abs() > 1 {
                t.0 += (h.0 - t.0).signum();
                t.1 += (h.1 - t.1).signum();
            }
            println!("{} {} {} {}", h.0, h.1, t.0, t.1);
            pos.insert(t);
        }
    }
    pos.len() as i64
}

pub fn solve_2() -> i64 {
    let mut k = [(0i64, 0i64); 10];
    let mut pos = HashSet::from([(0i64, 0i64)]);
    for line in read_lines("advent_2022/9.txt") {
        let p = line.split(' ').collect_vec();
        let d = p[0].as_bytes()[0];
        let l = p[1].parse::<i64>().unwrap();
        let dir = match d {
            b'U' => (0, 1),
            b'R' => (1, 0),
            b'D' => (0, -1),
            b'L' => (-1, 0),
            _ => panic!(),
        };
        for i in 0..l {
            k[0] = (k[0].0 + dir.0, k[0].1 + dir.1);
            for o in 1..10 {
                if (k[o - 1].0 - k[o].0).abs() > 1 || (k[o - 1].1 - k[o].1).abs() > 1 {
                    k[o].0 += (k[o - 1].0 - k[o].0).signum();
                    k[o].1 += (k[o - 1].1 - k[o].1).signum();
                }
            }
            pos.insert(k[9]);
        }
    }
    pos.len() as i64
}
