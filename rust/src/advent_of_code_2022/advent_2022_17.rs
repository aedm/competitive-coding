use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;

fn has_overlap(x: usize, y: usize, shape: &[(usize, usize)], grid: &[[bool; 7]]) -> bool {
    shape.iter().any(|&(sx, sy)| x + sx >= 7 || grid[y + sy][x + sx])
}

pub fn solve(count: usize) -> usize {
    let input = read_lines("advent_2022/17.txt")[0].bytes().map(|b| b == b'<').collect_vec();
    let shapes = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];
    let mut grid = vec![[false; 7]; 5_000];
    let mut wind = 0;
    let mut history = vec![];
    let mut lookup: HashMap<_, usize> = HashMap::new();
    let depth = 100;
    let mut i = 0;
    let mut skipped_height = 0;
    while i < count {
        let shi = i % shapes.len();
        let mut y = grid.iter().find_position(|row| row.iter().all(|&b| !b)).unwrap().0 + 3;
        let mut x = 2;

        if skipped_height == 0 && history.len() >= depth {
            let hk = history[i - depth..].iter().map(|&(ix, iy)| (ix, y - iy)).collect_vec();
            let key = (shi, wind, hk);
            if let Some(&p) = lookup.get(&key) {
                let diff = history[i - 1].1 - history[p].1;
                let skip_length = i - 1 - p;
                let skip_count = (count - i) / skip_length;
                skipped_height = skip_count * diff;
                i += skip_count * skip_length;
            } else {
                lookup.insert(key, i - 1);
            }
        }

        loop {
            let new_x = if input[wind] { max(x, 1) - 1 } else { min(x, 5) + 1 };
            wind = (wind + 1) % input.len();
            if !has_overlap(new_x, y, &shapes[shi], &grid) {
                x = new_x;
            }
            if y == 0 || has_overlap(x, y - 1, &shapes[shi], &grid) {
                break;
            }
            y -= 1;
        }
        for &(sx, sy) in &shapes[shi] {
            grid[y + sy][x + sx] = true;
        }
        history.push((x, y));
        i += 1;
    }
    grid.iter().find_position(|row| row.iter().all(|&b| !b)).unwrap().0 + skipped_height
}

pub fn solve_1() -> usize {
    solve(2022)
}
pub fn solve_2() -> usize {
    solve(1_000_000_000_000)
}
