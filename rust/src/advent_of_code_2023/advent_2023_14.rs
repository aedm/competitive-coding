use crate::utils::{map_add, read_lines};
use itertools::Itertools;
use std::collections::HashMap;

pub fn solve_d(cycles: usize) -> i64 {
    let mut m =
        read_lines("advent_2023/14.txt").iter().map(|l| l.chars().collect_vec()).collect_vec();
    let (w, h) = (m[0].len() as i64, m.len() as i64);

    let dirs = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut map_to_index = HashMap::new();
    let mut index_to_map = HashMap::<usize, Vec<Vec<char>>>::new();

    let mut i = 0;
    while i < cycles {
        let dir = i % 4;
        let mut rocks = Itertools::cartesian_product(0..w, 0..h)
            .filter(|&(x, y)| m[y as usize][x as usize] == 'O')
            .collect_vec();
        rocks.sort_by_key(|&(x, y)| (x * -dirs[dir].0, y * -dirs[dir].1));
        for start in rocks {
            let mut end = start;
            while let Some(c) = map_add(end, dirs[dir], w, h) {
                if m[c.1 as usize][c.0 as usize] != '.' {
                    break;
                }
                end = c;
            }
            m[start.1 as usize][start.0 as usize] = '.';
            m[end.1 as usize][end.0 as usize] = 'O';
        }
        if let Some(&prev_i) = map_to_index.get(&(m.clone(), dir)) {
            let skips = (cycles - i) / (i - prev_i);
            if skips > 0 {
                i += skips * (i - prev_i);
                m = index_to_map.get(&prev_i).unwrap().clone();
            }
        } else {
            map_to_index.insert((m.clone(), dir), i);
            index_to_map.insert(i, m.clone());
        }
        i += 1;
    }
    Itertools::cartesian_product(0..w, 0..h)
        .filter(|&(x, y)| m[y as usize][x as usize] == 'O')
        .map(|(x, y)| h - y)
        .sum::<i64>()
}

pub fn solve_1() -> i64 {
    solve_d(1)
}

pub fn solve() -> i64 {
    solve_d(4_000_000_000)
}
