use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

fn bt(
    c: usize,
    visited: &mut HashSet<usize>,
    caves: &[(String, bool)],
    adj: &HashMap<usize, Vec<usize>>,
) -> i64 {
    if c == 1 {
        return 1;
    }
    if !caves[c].1 {
        visited.insert(c);
    }
    let mut sum = 0;
    for &p in adj.get(&c).unwrap() {
        if !visited.contains(&p) {
            sum += bt(p, visited, caves, adj);
        }
    }
    visited.remove(&c);
    sum
}

pub fn solve_1() -> i64 {
    let lines = read_lines("advent_2021/12.txt");
    let mut caves = vec![("start".to_owned(), false), ("end".to_owned(), false)];
    let mut adj = HashMap::<usize, Vec<usize>>::new();
    for line in lines {
        let s = line
            .split('-')
            .map(|s| {
                if let Some(it) = caves.iter().find_position(|c| c.0 == s) {
                    it.0
                } else {
                    caves.push((s.to_owned(), s.chars().nth(0).unwrap().is_uppercase()));
                    caves.len() - 1
                }
            })
            .collect_vec();
        adj.entry(s[0]).or_insert_with(|| vec![]).push(s[1]);
        adj.entry(s[1]).or_insert_with(|| vec![]).push(s[0]);
    }

    let mut visited = HashSet::<usize>::new();

    bt(0, &mut visited, &caves, &adj)
}

fn bt2(
    c: usize,
    visited: &mut Vec<i64>,
    caves: &[(String, bool)],
    adj: &HashMap<usize, Vec<usize>>,
    hastwo: bool,
) -> i64 {
    if c == 1 {
        return 1;
    }
    if !caves[c].1 {
        visited[c] += 1;
    }
    let mut sum = 0;
    for &p in adj.get(&c).unwrap() {
        if p == 0 {
            continue;
        }
        if visited[p] == 0 {
            sum += bt2(p, visited, caves, adj, hastwo);
        } else {
            if !hastwo {
                sum += bt2(p, visited, caves, adj, true);
            }
        }
    }
    if !caves[c].1 {
        visited[c] -= 1;
    }
    sum
}

pub fn solve() -> i64 {
    let lines = read_lines("advent_2021/12.txt");
    let mut caves = vec![("start".to_owned(), false), ("end".to_owned(), false)];
    let mut adj = HashMap::<usize, Vec<usize>>::new();
    for line in lines {
        let s = line
            .split('-')
            .map(|s| {
                if let Some(it) = caves.iter().find_position(|c| c.0 == s) {
                    it.0
                } else {
                    caves.push((s.to_owned(), s.chars().nth(0).unwrap().is_uppercase()));
                    caves.len() - 1
                }
            })
            .collect_vec();
        adj.entry(s[0]).or_insert_with(|| vec![]).push(s[1]);
        adj.entry(s[1]).or_insert_with(|| vec![]).push(s[0]);
    }

    let mut visited = vec![0i64; caves.len()];

    bt2(0, &mut visited, &caves, &adj, false)
}
