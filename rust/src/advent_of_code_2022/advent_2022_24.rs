use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve_1() -> isize {
    let lines = read_lines("advent_2022/24.txt");
    let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)];
    let dir_signs = HashMap::from([('^', 0), ('v', 1), ('<', 2), ('>', 3)]);
    let mut s = vec![];
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(&d) = dir_signs.get(&c) {
                s.push((x as isize, y as isize, d));
            }
        }
    }
    let (w, h) = (lines[0].len() as isize - 2, lines.len() as isize - 2);
    let (ex, ey) = (w, h + 1);
    let mut pos = HashSet::from([(1, 0)]);
    for i in 1.. {
        println!("{}: {}", i, pos.len());
        for b in &mut s {
            let (nx, ny) = (b.0 + dirs[b.2].0, b.1 + dirs[b.2].1);
            let nx = (nx + w - 1) % w + 1;
            let ny = (ny + h - 1) % h + 1;
            *b = (nx, ny, b.2);
        }
        let occupied = s.iter().map(|(x, y, _)| (*x, *y)).collect::<HashSet<_>>();
        let mut new_pos = HashSet::new();
        for (x, y) in pos {
            for &(dx, dy) in &dirs {
                let (nx, ny) = (x + dx, y + dy);
                if (nx, ny) == (ex, ey) {
                    return i;
                }
                if ((x, y) == (nx, ny) || (nx > 0 && ny > 0 && nx < w + 1 && ny < h + 1))
                    && !occupied.contains(&(nx, ny))
                {
                    new_pos.insert((nx, ny));
                }
            }
        }
        pos = new_pos;
    }
    unreachable!()
}

pub fn solve_2() -> isize {
    let lines = read_lines("advent_2022/24.txt");
    let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)];
    let dir_signs = HashMap::from([('^', 0), ('v', 1), ('<', 2), ('>', 3)]);
    let mut s = vec![];
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(&d) = dir_signs.get(&c) {
                s.push((x as isize, y as isize, d));
            }
        }
    }
    let (w, h) = (lines[0].len() as isize - 2, lines.len() as isize - 2);
    let (mut ex, mut ey) = (w, h + 1);
    let mut pos = HashSet::from([(1, 0)]);
    let mut state = 0;
    'lp: for i in 1.. {
        for b in &mut s {
            let (nx, ny) = (b.0 + dirs[b.2].0, b.1 + dirs[b.2].1);
            let nx = (nx + w - 1) % w + 1;
            let ny = (ny + h - 1) % h + 1;
            *b = (nx, ny, b.2);
        }
        let occupied = s.iter().map(|(x, y, _)| (*x, *y)).collect::<HashSet<_>>();
        let mut new_pos = HashSet::new();
        for (x, y) in pos {
            for &(dx, dy) in &dirs {
                let (nx, ny) = (x + dx, y + dy);
                if (nx, ny) == (ex, ey) {
                    pos = HashSet::from([(ex, ey)]);
                    (ex, ey) = match state {
                        0 => (1, 0),
                        1 => (w, h + 1),
                        _ => return i,
                    };
                    state += 1;
                    continue 'lp;
                }
                if ((x, y) == (nx, ny) || (nx > 0 && ny > 0 && nx < w + 1 && ny < h + 1))
                    && !occupied.contains(&(nx, ny))
                {
                    new_pos.insert((nx, ny));
                }
            }
        }
        pos = new_pos;
    }
    unreachable!()
}
