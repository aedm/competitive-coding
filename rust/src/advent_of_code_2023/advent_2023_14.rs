use crate::utils::array2d::{DIRS4, Map2D};
use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(cycles: usize) -> i64 {
    let mut m = Map2D::from_text(&read_lines("advent_2023/14.txt"));
    let mut map_to_index = HashMap::new();
    let mut maps = vec![];
    for i in 0..cycles {
        let dir = i % 4;
        let rocks = m
            .filter(|_, v| *v == 'O')
            .map(|(p, _)| p)
            .sorted_by_key(|&c| c * -DIRS4[dir])
            .collect_vec();
        for mut rock in rocks {
            m[rock] = '.';
            while let Some((c, '.')) = m.add_coord(rock, DIRS4[dir]) {
                rock = c;
            }
            m[rock] = 'O';
        }
        let key = (m.clone(), dir);
        if let Some(&prev_i) = map_to_index.get(&key) {
            m = Map2D::clone(&maps[prev_i + (cycles - i) % (i - prev_i) - 1]);
            break;
        }
        map_to_index.insert(key, i);
        maps.push(m.clone());
    }
    m.filter(|_, v| *v == 'O').map(|(p, _)| m.h - p.y).sum()
}

pub fn solve_1() -> i64 {
    solve(1)
}

pub fn solve_2() -> i64 {
    solve(4_000_000_000)
}
