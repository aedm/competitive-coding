use crate::utils::{map_add, read_lines};
use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(cycles: usize) -> i64 {
    let mut m: Vec<Vec<char>> =
        read_lines("advent_2023/14.txt").iter().map(|l| l.chars().collect_vec()).collect_vec();
    let (w, h) = (m[0].len() as i64, m.len() as i64);

    let dirs = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut map_to_index = HashMap::new();
    let mut maps = vec![];

    for i in 0..cycles {
        let dir = i % 4;
        let rocks = Itertools::cartesian_product(0..w, 0..h)
            .filter(|&(x, y)| m[y as usize][x as usize] == 'O')
            .sorted_by_key(|&(x, y)| (x * -dirs[dir].0, y * -dirs[dir].1))
            .collect_vec();
        for mut rock in rocks {
            m[rock.1 as usize][rock.0 as usize] = '.';
            while let Some(c) = map_add(rock, dirs[dir], w, h) {
                if m[c.1 as usize][c.0 as usize] != '.' {
                    break;
                }
                rock = c;
            }
            m[rock.1 as usize][rock.0 as usize] = 'O';
        }
        if let Some(&prev_i) = map_to_index.get(&(m.clone(), dir)) {
            m = std::mem::take(&mut maps[prev_i + (cycles - i) % (i - prev_i) - 1]);
            break;
        }
        map_to_index.insert((m.clone(), dir), i);
        maps.push(m.clone());
    }
    Itertools::cartesian_product(0..w, 0..h)
        .filter(|&(x, y)| m[y as usize][x as usize] == 'O')
        .map(|(x, y)| h - y)
        .sum::<i64>()
}

pub fn solve_1() -> i64 {
    solve(1)
}

pub fn solve_2() -> i64 {
    solve(4_000_000_000)
}
