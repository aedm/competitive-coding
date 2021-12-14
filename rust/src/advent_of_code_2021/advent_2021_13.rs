use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

pub fn solve_1() -> i64 {
    let mut lines = read_lines("advent_2021/13.txt").into_iter();
    let mut dots = lines
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|s| s.split(',').map(|n| n.parse::<i64>().unwrap()).collect_vec())
        .map(|v| (v[0], v[1]))
        .collect_vec();
    let folds = lines
        .map(|s| {
            let m = s.split('=').collect_vec();
            (m[0] == "fold along x", m[1].parse::<i64>().unwrap())
        })
        .collect_vec();

    for &(is_x, c) in folds.iter().take(1) {
        if is_x {
            dots.iter_mut().for_each(|d| d.0 = c - (c - d.0).abs());
        } else {
            dots.iter_mut().for_each(|d| d.1 = c - (c - d.1).abs());
        }
    }
    let dp: HashSet<_> = dots.iter().collect();
    dp.len() as i64
}

pub fn solve_2() -> i64 {
    let mut lines = read_lines("advent_2021/13.txt").into_iter();
    let mut dots = lines
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|s| s.split(',').map(|n| n.parse::<i64>().unwrap()).collect_vec())
        .map(|v| (v[0], v[1]))
        .collect_vec();
    let folds = lines
        .map(|s| {
            let m = s.split('=').collect_vec();
            (m[0] == "fold along x", m[1].parse::<i64>().unwrap())
        })
        .collect_vec();

    for &(is_x, c) in &folds {
        if is_x {
            dots.iter_mut().for_each(|d| d.0 = c - (c - d.0).abs());
        } else {
            dots.iter_mut().for_each(|d| d.1 = c - (c - d.1).abs());
        }
    }
    let dp: HashSet<_> = dots.iter().collect();

    let mx = dp.iter().map(|d| d.0).max().unwrap();
    let my = dp.iter().map(|d| d.1).max().unwrap();
    let mut x = vec![vec![0; mx as usize + 1]; my as usize + 1];
    for &&d in &dp {
        x[d.1 as usize][d.0 as usize] = 1;
    }
    x.iter().for_each(|l| println!("{:?}", l));

    dp.len() as i64
}
