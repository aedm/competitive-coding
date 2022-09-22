use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::max;
use std::collections::{BTreeSet, VecDeque};
use std::iter::FromIterator;

pub fn solve_1() -> i64 {
    let lines = read_lines("advent_2021/10.txt");
    let pairs = [
        ('(', ')', 3),
        ('[', ']', 57),
        ('{', '}', 1197),
        ('<', '>', 25137),
    ];
    let mut sum = 0;
    for line in lines {
        let mut v = vec![];
        for c in line.chars() {
            if let Some(&p) = pairs.iter().find(|p| p.1 == c) {
                if Some(&p.0) != v.last() {
                    sum += p.2;
                    break;
                }
                v.pop();
            } else {
                v.push(c);
            }
        }
    }
    sum
}

pub fn solve_2() -> i64 {
    let lines = read_lines("advent_2021/10.txt");
    let pairs = [('(', ')', 1), ('[', ']', 2), ('{', '}', 3), ('<', '>', 4)];
    let mut scores = vec![];
    'mainloop: for line in lines {
        let mut v = vec![];
        for c in line.chars() {
            if let Some(p) = pairs.iter().find(|p| p.1 == c) {
                if Some(&p.0) != v.last() {
                    continue 'mainloop;
                }
                v.pop();
            } else {
                v.push(c);
            }
        }
        let mut score = v.iter().rfold(0, |acc, &c| {
            acc * 5 + pairs.iter().find(|p| p.0 == c).unwrap().2
        });
        scores.push(score);
    }
    scores.sort();
    scores[scores.len() / 2]
}
