use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

fn solve(steps: i64) -> i64 {
    let mut lines = read_lines("advent_2021/14.txt");
    let mut v = lines[0].chars().collect_vec();
    let rules: HashMap<(char, char), char> = lines[2..]
        .iter()
        .map(|l| {
            let a = l.chars().collect_vec();
            ((a[0], a[1]), a[6])
        })
        .collect();

    let mut pairs = HashMap::<(char, char), i64>::new();
    for p in 1..v.len() {
        *pairs.entry((v[p - 1], v[p])).or_insert(0) += 1;
    }

    let mut counts = HashMap::<char, i64>::new();
    for p in 0..v.len() {
        *counts.entry(v[p]).or_insert(0) += 1;
    }

    for i in 0..steps {
        let mut p = HashMap::<(char, char), i64>::new();
        for (&c, &n) in &pairs {
            if let Some(&nc) = rules.get(&(c.0, c.1)) {
                *p.entry((c.0, nc)).or_insert(0) += n;
                *p.entry((nc, c.1)).or_insert(0) += n;
                *counts.entry(nc).or_insert(0) += n;
            } else {
                *p.entry((c.0, c.1)).or_insert(0) += n;
            }
        }
        pairs = p;
    }

    let max = counts.iter().max_by_key(|it| it.1).unwrap();
    let min = counts.iter().min_by_key(|it| it.1).unwrap();
    max.1 - min.1
}

pub fn solve_1() -> i64 {
    solve(10)
}

pub fn solve_2() -> i64 {
    solve(40)
}
