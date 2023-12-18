use crate::utils::array2d::{v, IVec2D, DIRS4};
use crate::utils::read_lines_split;
use itertools::Itertools;
use std::collections::HashMap;

fn area(ps: &[(IVec2D, usize)]) -> i64 {
    let mut a = 0;
    for i in 0..ps.len() {
        let (p1, p2) = (ps[i].0, ps[(i + 1) % ps.len()].0);
        a += (p1.x * p2.y) - (p1.y * p2.x)
    }
    a / 2
}

pub fn solve_inner(ins: &[(i64, usize)]) -> i64 {
    let mut p = v(0, 0);
    let mut ps = vec![];
    for &(l, d) in ins {
        ps.push((p, d));
        p += DIRS4[d] * l;
    }
    let outline = [v(1, 0), v(0, 0), v(0, 0), v(0, 1)];
    let s = area(&ps).signum();
    let mut d_prev = ps[ps.len() - 1].1;
    for (p, d) in ps.iter_mut() {
        *p -= (outline[*d] + outline[d_prev]) * s;
        d_prev = *d;
    }
    area(&ps)
}

pub fn solve_1() -> i64 {
    let ds = HashMap::from([(b'R', 3), (b'L', 1), (b'U', 0), (b'D', 2)]);
    let l = read_lines_split("advent_2023/18.txt", &[' '])
        .iter()
        .map(|l| {
            let a = l[1].parse::<i64>().unwrap();
            (a, ds[&l[0].as_bytes()[0]])
        })
        .collect_vec();
    solve_inner(&l)
}

pub fn solve_2() -> i64 {
    let ds = HashMap::from([(b'0', 3), (b'1', 2), (b'2', 1), (b'3', 0)]);
    let l = read_lines_split("advent_2023/18.txt", &[' '])
        .iter()
        .map(|l| {
            let a = i64::from_str_radix(&l[2][2..7], 16).unwrap();
            (a, ds[&l[2].as_bytes()[7]])
        })
        .collect_vec();
    solve_inner(&l)
}
