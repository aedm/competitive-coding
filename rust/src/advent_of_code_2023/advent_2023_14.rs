use crate::utils::array2d::{Map2D, DIRS4};
use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::HashMap;

pub fn solve_orig(cycles: usize) -> i64 {
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

pub fn solve(cycles: usize) -> i64 {
    let mut m = Map2D::from_text_u8(&read_lines("advent_2023/14.txt"));
    let now = std::time::Instant::now();
    let mut rocks_indices = HashMap::new();
    let mut history_of_rock = vec![];
    let mut rocks = m.filter(|_, v| *v == b'O').map(|(p, _)| p).collect_vec();
    for i in 0..cycles {
        let dir = DIRS4[i % 4];
        rocks.sort_by_key(|&c| c * -dir);
        let mut new_rocks = Vec::with_capacity(rocks.len());
        for mut rock in rocks {
            m[rock] = b'.';
            while let Some((c, b'.')) = m.add_coord(rock, dir) {
                rock = c;
            }
            m[rock] = b'O';
            new_rocks.push(rock);
        }
        rocks = new_rocks;
        if i % 4 == 3 {
            let cc = i/4;
            if let Some(&prev_i) = rocks_indices.get(&rocks) {
                rocks = Vec::clone(&history_of_rock[prev_i + (cycles / 4 - cc) % (cc - prev_i) - 1]);
                break;
            }
            rocks_indices.insert(rocks.clone(), cc);
            history_of_rock.push(rocks.clone());
        }
    }
    let r =  rocks.iter().map(|p| m.h - p.y).sum();
    println!("Elapsed without IO: {:?}", now.elapsed());
    r
}

pub fn solve_1() -> i64 {
    solve(1)
}

pub fn solve_2() -> i64 {
    solve(4_000_000_000)
}
