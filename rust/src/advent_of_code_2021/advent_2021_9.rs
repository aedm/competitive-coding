use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::max;
use std::collections::{BTreeSet, VecDeque};
use std::iter::FromIterator;

pub fn solve_1() -> i64 {
    let lines = read_lines("advent_2021/9.txt");
    let n = lines.iter().map(|s| s.bytes().map(|b| (b - b'0') as i64).collect_vec()).collect_vec();
    let dir = [(1i64, 0i64), (-1, 0), (0, 1), (0, -1)];
    let xlen = n[0].len() as i64;
    let ylen = n.len() as i64;
    let mins = ((0..xlen).cartesian_product(0..ylen))
        .filter(|&(x, y)| {
            dir.iter().all(|&(dx, dy)| {
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
    let dir = [(1i64, 0i64), (-1, 0), (0, 1), (0, -1)];
    let xlen = n[0].len() as i64;
    let ylen = n.len() as i64;

    let mut basins = vec![];

    let mut vis = BTreeSet::new();
    for (x, y) in (0..xlen).cartesian_product(0..ylen) {
        if n[y as usize][x as usize] == 9 {
            vis.insert((x, y));
        }
    }

    while (vis.len() as i64) < xlen * ylen {
        let mut unalloc = vis.len();
        let mut q = VecDeque::new();
        for (x, y) in (0..xlen).cartesian_product(0..ylen) {
            if !vis.contains(&(x, y)) {
                vis.insert((x, y));
                q.push_back((x, y));
                break;
            }
        }

        while let Some(c) = q.pop_front() {
            for &(dx, dy) in &dir {
                let (nx, ny) = (c.0 + dx, c.1 + dy);
                if nx == nx.clamp(0, xlen - 1)
                    && ny == ny.clamp(0, ylen - 1)
                    && !vis.contains(&(nx, ny))
                {
                    vis.insert((nx, ny));
                    q.push_back((nx, ny));
                }
            }
        }
        basins.push(vis.len() - unalloc);
    }
    basins.iter().sorted().rev().take(3).product::<usize>()
}
