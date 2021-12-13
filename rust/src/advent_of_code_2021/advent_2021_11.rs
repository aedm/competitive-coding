use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{BTreeSet, VecDeque};
use std::iter::FromIterator;

pub fn solve_1() -> i64 {
    let lines = read_lines("advent_2021/11.txt");
    let mut n =
        lines.iter().map(|l| l.bytes().map(|b| (b - b'0') as i64).collect_vec()).collect_vec();

    let mut flashes = 0;
    for _ in 0..100 {
        ((0..10).cartesian_product(0..10)).for_each(|(x, y)| n[y][x] += 1);
        let mut changed = true;
        while changed {
            changed = false;
            for (x, y) in (0i64..10).cartesian_product(0i64..10) {
                if n[y as usize][x as usize] > 9 {
                    n[y as usize][x as usize] = 0;
                    changed = true;
                    flashes += 1;
                    for (nx, ny) in (max(x - 1, 0)..=min(x + 1, 9))
                        .cartesian_product(max(y - 1, 0)..=min(y + 1, 9))
                    {
                        if n[ny as usize][nx as usize] > 0 {
                            n[ny as usize][nx as usize] += 1;
                        }
                    }
                }
            }
        }
    }
    flashes
}

pub fn solve_2() -> i64 {
    let lines = read_lines("advent_2021/11.txt");
    let mut n =
        lines.iter().map(|l| l.bytes().map(|b| (b - b'0') as i64).collect_vec()).collect_vec();

    for step in 1..1000000 {
        ((0..10).cartesian_product(0..10)).for_each(|(x, y)| n[y][x] += 1);
        let mut changed = true;
        let mut flashes = 0;
        while changed {
            changed = false;
            for (x, y) in (0i64..10).cartesian_product(0i64..10) {
                if n[y as usize][x as usize] > 9 {
                    n[y as usize][x as usize] = 0;
                    changed = true;
                    flashes += 1;
                    for (nx, ny) in (max(x - 1, 0)..=min(x + 1, 9))
                        .cartesian_product(max(y - 1, 0)..=min(y + 1, 9))
                    {
                        if n[ny as usize][nx as usize] > 0 {
                            n[ny as usize][nx as usize] += 1;
                        }
                    }
                }
            }
        }
        if flashes == 100 {
            return step;
        }
    }
    panic!()
}
