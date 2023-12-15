use crate::utils::{map_add, read_lines, read_lines_split};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

fn read_input() -> ((i64, i64), Vec<Vec<[u8; 4]>>) {
    let p = HashMap::from([
        ('.', [0, 0, 0, 0]),
        ('|', [1, 0, 1, 0]),
        ('-', [0, 1, 0, 1]),
        ('S', [1, 1, 1, 1]),
        ('L', [1, 1, 0, 0]),
        ('J', [1, 0, 0, 1]),
        ('7', [0, 0, 1, 1]),
        ('F', [0, 1, 1, 0]),
    ]);
    let lines = read_lines("advent_2023/10.txt");
    let mut s = (0, 0);
    let mut m = vec![vec![[0; 4]; lines[0].len()]; lines.len()];
    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                s = (x as i64, y as i64);
            }
            m[y][x] = p[&c];
        }
    }
    (s, m)
}

fn find_loop(m: &Vec<Vec<[u8; 4]>>, s: (i64, i64)) -> Vec<Vec<usize>> {
    let d = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut q = VecDeque::new();
    q.push_back((s, 0));
    let mut dist = vec![vec![std::usize::MAX; m[0].len()]; m.len()];
    while let Some(((x, y), ds)) = q.pop_front() {
        if dist[y as usize][x as usize] != std::usize::MAX {
            continue;
        }
        dist[y as usize][x as usize] = ds;
        for di in 0..4 {
            if m[y as usize][x as usize][di] != 0 {
                if let Some(n) = map_add((x, y), d[di], m[0].len() as i64, m.len() as i64)
                {
                    if m[n.1 as usize][n.0 as usize][(di + 2) % 4] == 1 {
                        q.push_back((n, ds + 1));
                    }
                }
            }
        }
    }
    dist
}

pub fn solve_1() -> impl Debug {
    let (s, m) = read_input();
    let dist = find_loop(&m, s);
    dist.iter().flat_map(|r| r.iter().filter(|k| **k < std::usize::MAX)).max().cloned().unwrap()
}

pub fn solve_2() -> impl Debug {
    let d = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let (s, m) = read_input();

    let mut m2 = vec![vec![[0; 4]; m[0].len() * 2]; m.len() * 2];
    for y in 0..m.len() {
        for x in 0..m[0].len() {
            m2[y * 2][x * 2] = m[y][x];
            if x < m[0].len() - 1 && m[y][x][1] == 1 && m[y][x + 1][3] == 1 {
                m2[y * 2][x * 2 + 1][1] = 1;
                m2[y * 2][x * 2 + 1][3] = 1;
            }
            if y < m.len() - 1 && m[y][x][2] == 1 && m[y + 1][x][0] == 1 {
                m2[y * 2 + 1][x * 2][2] = 1;
                m2[y * 2 + 1][x * 2][0] = 1;
            }
        }
    }

    let lp = find_loop(&m2, (s.0 * 2, s.1 * 2));
    let mut q = (0..m2.len() as i64)
        .cartesian_product(0..m2[0].len() as i64)
        .filter(|(y, x)| {
            *x == 0 || *y == 0 || *x == m2[0].len() as i64 - 1 || *y == m2.len() as i64 - 1
        })
        .collect::<VecDeque<_>>();

    let mut outside = vec![vec![false; m2[0].len()]; m2.len()];
    while let Some((x, y)) = q.pop_front() {
        if outside[y as usize][x as usize] || lp[y as usize][x as usize] < std::usize::MAX {
            continue;
        }
        outside[y as usize][x as usize] = true;
        for di in 0..4 {
            if let Some(n) = map_add((x, y), d[di], m2[0].len() as i64, m2.len() as i64) {
                q.push_back(n);
            }
        }
    }

    (0..m.len())
        .cartesian_product(0..m[0].len())
        .filter(|(y, x)| !outside[2 * y][2 * x] && lp[2 * y][2 * x] == std::usize::MAX)
        .count()
}
