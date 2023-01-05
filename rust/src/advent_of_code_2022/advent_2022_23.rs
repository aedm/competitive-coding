use crate::utils::{debug_2d_map, debug_2d_map_fixed, read_lines};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve_1() -> isize {
    let mut taken = HashSet::new();
    for (y, line) in read_lines("advent_2022/23.txt").into_iter().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            if c == b'#' {
                taken.insert((x as isize, y as isize));
            }
        }
    }
    let mut dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    for _ in 0..10 {
        let mut proposed = HashMap::new();
        let mut prop_map = HashMap::new();
        for &(x, y) in taken.iter() {
            if (-1..=1)
                .cartesian_product(-1..=1)
                .filter(|&(nx, ny)| taken.contains(&(x + nx, y + ny)))
                .count()
                == 1
            {
                continue;
            }
            for di in 0..4 {
                if !taken.contains(&(x + dirs[di].0, y + dirs[di].1))
                    && !taken.contains(&(x + dirs[di].0 + dirs[di].1, y + dirs[di].1 + dirs[di].0))
                    && !taken.contains(&(x + dirs[di].0 - dirs[di].1, y + dirs[di].1 - dirs[di].0))
                {
                    let new = (x + dirs[di].0, y + dirs[di].1);
                    proposed.insert((x, y), new);
                    *prop_map.entry(new).or_insert(0) += 1;
                    break;
                }
            }
        }
        for (from, to) in proposed {
            if *prop_map.get(&to).unwrap() == 1 {
                taken.remove(&from);
                taken.insert(to);
            }
        }
        for z in 0..3 {
            dirs.swap(z, z + 1);
        }
    }
    let (min_x, max_x, min_y, max_y) = taken.iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(min_x, max_x, min_y, max_y), (x, y)| {
            (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
        },
    );
    (max_x - min_x + 1) * (max_y - min_y + 1) - taken.len() as isize
}

pub fn solve_2() -> isize {
    let mut taken = HashSet::new();
    for (y, line) in read_lines("advent_2022/23.txt").into_iter().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            if c == b'#' {
                taken.insert((x as isize, y as isize));
            }
        }
    }
    let mut dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    for i in 1.. {
        let mut proposed = HashMap::new();
        let mut prop_map = HashMap::new();
        for &(x, y) in taken.iter() {
            if (-1..=1)
                .cartesian_product(-1..=1)
                .filter(|&(nx, ny)| taken.contains(&(x + nx, y + ny)))
                .count()
                == 1
            {
                continue;
            }
            for di in 0..4 {
                if !taken.contains(&(x + dirs[di].0, y + dirs[di].1))
                    && !taken.contains(&(x + dirs[di].0 + dirs[di].1, y + dirs[di].1 + dirs[di].0))
                    && !taken.contains(&(x + dirs[di].0 - dirs[di].1, y + dirs[di].1 - dirs[di].0))
                {
                    let new = (x + dirs[di].0, y + dirs[di].1);
                    proposed.insert((x, y), new);
                    *prop_map.entry(new).or_insert(0) += 1;
                    break;
                }
            }
        }
        let mut moves = 0;
        for (from, to) in proposed {
            if *prop_map.get(&to).unwrap() == 1 {
                taken.remove(&from);
                taken.insert(to);
                moves += 1;
            }
        }
        if moves == 0 {
            return i;
        }
        for z in 0..3 {
            dirs.swap(z, z + 1);
        }
    }
    unreachable!()
}
