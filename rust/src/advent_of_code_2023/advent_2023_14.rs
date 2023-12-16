use crate::utils::array2d::{Map2D, DIRS4};
use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(cycles: usize) -> i64 {
    let mut m = Map2D::from_text_u8(&read_lines("advent_2023/14.txt"));
    let mut rocks_indices = HashMap::new();
    let mut history_of_rock = vec![];
    let mut rocks = m.filter(|_, v| *v == b'O').map(|(p, _)| p).collect_vec();
    for i in 0..cycles {
        let dir = DIRS4[i % 4];
        rocks.sort_by_key(|&c| c * -dir);
        if i % 4 == 0 {
            if let Some(&pi) = rocks_indices.get(&rocks) {
                rocks = Vec::clone(&history_of_rock[pi + (cycles - i) / 4 % (i / 4 - pi)]);
                break;
            }
            rocks_indices.insert(rocks.clone(), i / 4);
            history_of_rock.push(rocks.clone());
        }
        for mut rock in rocks.iter_mut() {
            m[*rock] = b'.';
            while let Some((c, b'.')) = m.add_coord(*rock, dir) {
                *rock = c;
            }
            m[*rock] = b'O';
        }
    }
    rocks.iter().map(|p| m.h - p.y).sum()
}

pub fn solve_1() -> i64 {
    solve(1)
}

pub fn solve_2() -> i64 {
    solve(4_000_000_000)
}
