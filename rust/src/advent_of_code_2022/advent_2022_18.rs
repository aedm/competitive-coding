use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::VecDeque;

const DIR6: [(isize, isize, isize); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

pub fn solve_1() -> usize {
    let input = read_lines("advent_2022/18.txt")
        .into_iter()
        .map(|s| s.split(',').map(|p| p.parse::<usize>().unwrap() + 1).collect_vec())
        .collect_vec();
    let s = *input.iter().flatten().max().unwrap() + 2;
    let mut grid = vec![false; s * s * s];
    for p in &input {
        grid[p[0] + p[1] * s + p[2] * s * s] = true;
    }
    (1..s as isize - 1)
        .cartesian_product(1..s as isize - 1)
        .cartesian_product(1..s as isize - 1)
        .map(|((x, y), z)| {
            if !grid[x as usize + y as usize * s + z as usize * s * s] {
                return 0;
            }
            DIR6.iter()
                .filter(|&(dx, dy, dz)| {
                    let (x, y, z) = ((x + dx) as usize, (y + dy) as usize, (z + dz) as usize);
                    !grid[x + y * s + z * s * s]
                })
                .count()
        })
        .sum()
}

pub fn solve_2() -> usize {
    let input = read_lines("advent_2022/18.txt")
        .into_iter()
        .map(|s| s.split(',').map(|p| p.parse::<usize>().unwrap() + 1).collect_vec())
        .collect_vec();
    let s = *input.iter().flatten().max().unwrap() + 2;
    let mut grid = vec![0; s * s * s];
    for p in &input {
        grid[p[0] + p[1] * s + p[2] * s * s] = 1;
    }
    grid[0] = 2;
    let mut queue = VecDeque::from([(0, 0, 0)]);
    while let Some(p) = queue.pop_front() {
        for &(dx, dy, dz) in &DIR6 {
            let (x, y, z) = (p.0 as isize + dx, p.1 as isize + dy, p.2 as isize + dz);
            if !(x < 0 || y < 0 || z < 0 || x >= s as isize || y >= s as isize || z >= s as isize) {
                let (x, y, z) = (x as usize, y as usize, z as usize);
                if grid[x + y * s + z * s * s] == 0 {
                    grid[x + y * s + z * s * s] = 2;
                    queue.push_back((x, y, z));
                }
            }
        }
    }

    (1..s as isize - 1)
        .cartesian_product(1..s as isize - 1)
        .cartesian_product(1..s as isize - 1)
        .map(|((x, y), z)| {
            if grid[x as usize + y as usize * s + z as usize * s * s] != 1 {
                return 0;
            }
            DIR6.iter()
                .filter(|&(dx, dy, dz)| {
                    let (x, y, z) = ((x + dx) as usize, (y + dy) as usize, (z + dz) as usize);
                    grid[x + y * s + z * s * s] == 2
                })
                .count()
        })
        .sum()
}
