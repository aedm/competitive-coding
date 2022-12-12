use crate::utils::{neighbours4, read_lines};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve_1() -> i64 {
    let (mut px, mut py, mut ex, mut ey) = (0, 0, 0, 0);
    let m = read_lines("advent_2022/12.txt")
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, c)| match c {
                    b'S' => {
                        px = x;
                        py = y;
                        0
                    }
                    b'E' => {
                        ex = x;
                        ey = y;
                        b'z' - b'a'
                    }
                    c => c - b'a',
                })
                .collect_vec()
        })
        .collect_vec();
    let (w, h) = (m[0].len(), m.len());
    let mut queue = VecDeque::from(vec![(px, py)]);
    let mut visited = HashMap::from([((px, py), (px, py))]);
    'lp: while let Some(p) = queue.pop_front() {
        let ch = m[p.1][p.0];
        for (x, y) in neighbours4((p.0, p.1), 0, 0, w, h) {
            if m[y][x] <= ch + 1 && !visited.contains_key(&(x, y)) {
                visited.insert((x, y), p);
                queue.push_back((x, y));
                if x == ex && y == ey {
                    break 'lp;
                }
            }
        }
    }
    let mut steps = 0;
    while (ex, ey) != (px, py) {
        (ex, ey) = visited[&(ex, ey)];
        steps += 1;
    }
    steps
}

pub fn solve_2() -> i64 {
    let (mut ex, mut ey) = (0, 0);
    let m = read_lines("advent_2022/12.txt")
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, c)| match c {
                    b'S' => 0,
                    b'E' => {
                        ex = x;
                        ey = y;
                        b'z' - b'a'
                    }
                    c => c - b'a',
                })
                .collect_vec()
        })
        .collect_vec();
    let (w, h) = (m[0].len(), m.len());
    let mut queue = VecDeque::from(vec![(ex, ey)]);
    let mut visited = HashMap::from([((ex, ey), 0)]);
    while let Some(p) = queue.pop_front() {
        let ch = m[p.1][p.0];
        let steps = visited[&p];
        for (x, y) in neighbours4((p.0, p.1), 0, 0, w, h) {
            if m[y][x] + 1 >= ch && !visited.contains_key(&(x, y)) {
                visited.insert((x, y), steps + 1);
                queue.push_back((x, y));
            }
        }
    }
    (0..w)
        .cartesian_product(0..h)
        .filter(|(x, y)| m[*y][*x] == 0 && visited.contains_key(&(*x, *y)))
        .map(|(x, y)| visited[&(x, y)])
        .min()
        .unwrap()
}
