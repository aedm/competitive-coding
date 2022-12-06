use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve_1() -> i64 {
    read_lines("advent_2022/3.txt")
        .iter()
        .map(|l| {
            let n = l
                .bytes()
                .map(|c| if c.is_ascii_lowercase() { c - b'a' + 1 } else { c - b'A' + 27 })
                .collect_vec();
            let r1 = n[0..n.len() / 2].iter().collect::<HashSet<_>>();
            let r2 = n[n.len() / 2..].iter().collect::<HashSet<_>>();
            **r1.intersection(&r2).nth(0).unwrap() as i64
        })
        .sum()
}

pub fn solve_2() -> i64 {
    read_lines("advent_2022/3.txt")
        .iter()
        .map(|l| {
            l.bytes()
                .map(|c| if c.is_ascii_lowercase() { c - b'a' + 1 } else { c - b'A' + 27 })
                .collect::<HashSet<_>>()
        })
        .chunks(3)
        .into_iter()
        .map(|mut ch| {
            let r1 = ch.next().unwrap().intersection(&ch.next().unwrap()).cloned().collect();
            *ch.next().unwrap().intersection(&r1).nth(0).unwrap() as i64
        })
        .sum()
}
