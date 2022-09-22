use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::max;
use std::collections::{BTreeSet, VecDeque};
use std::iter::FromIterator;

pub fn solve_1() -> i64 {
    let lines = read_lines("advent_2021/9.txt");
    let n = lines.iter().map(|s| s.bytes().map(|b| (b - b'0') as i64).collect_vec()).collect_vec();
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let xlen = n[0].len() as i64;
    let ylen = n.len() as i64;
    let mins = ((0..xlen).cartesian_product(0..ylen))
        .filter(|&(x, y)| {
            dirs.iter().all(|&(dx, dy)| {
                (x + dx) != (x + dx).clamp(0, xlen - 1)
                    || (y + dy) != (y + dy).clamp(0, ylen - 1)
                    || n[(y + dy) as usize][(x + dx) as usize] > n[y as usize][x as usize]
            })
        })
        .collect_vec();
    mins.iter().map(|&(x, y)| n[y as usize][x as usize]).sum::<i64>() + mins.len() as i64
}

pub fn solve_2() -> usize {
    let lines = read_lines("advent_2021/9.txt");
    let n = lines.iter().map(|s| s.bytes().map(|b| (b - b'0') as i64).collect_vec()).collect_vec();
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut dry_fields: BTreeSet<_> = ((0..n[0].len() as i64).cartesian_product(0..n.len() as i64))
        .filter(|&(x, y)| n[y as usize][x as usize] < 9)
        .collect();

    let mut basins = vec![];
    while let Some(e) = dry_fields.iter().cloned().next() {
        let mut flood_count = dry_fields.len();
        let mut queue = VecDeque::new();
        queue.push_back(e);
        dry_fields.remove(&e);
        while let Some(c) = queue.pop_front() {
            for &(dx, dy) in &dirs {
                let nc = (c.0 + dx, c.1 + dy);
                dry_fields.remove(&nc).then(|| queue.push_back(nc));
            }
        }
        basins.push(flood_count - dry_fields.len());
    }
    basins.iter().sorted().rev().take(3).product::<usize>()
}
