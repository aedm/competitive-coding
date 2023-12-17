use crate::utils::array2d::{IVec2D, Map2D, DIRS4};
use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::VecDeque;

fn solve(mins: usize, maxs: usize) -> i64 {
    let m = Map2D::from_text(&read_lines("advent_2023/17.txt"));
    let mut start_coord = IVec2D::new(0, 0);
    let mut end_coord = IVec2D::new(m.w - 1, m.h - 1);

    let mut m = Map2D::from_map(&m, |v| (*v as i64 - '0' as i64));
    let mut n = Map2D::from_map(&m, |_| [[i64::MAX; 11]; 4]);
    let mut queue = VecDeque::from([(start_coord, 0, 2, maxs), (start_coord, 0, 3, maxs)]);
    while let Some((c, v, d, t)) = queue.pop_front() {
        if n[c][d][t] > v {
            n[c][d][t] = v;
            for nd in 0..4 {
                if !(nd == (d + 2) % 4 || (nd == d && t == 0) || (nd != d && t > maxs - mins)) {
                    let nt = if nd == d { t - 1 } else { maxs - 1 };
                    if let Some((nc, _)) = m.add_coord(c, DIRS4[nd]) {
                        queue.push_back((nc, v + m[nc], nd, nt));
                    }
                }
            }
        }
    }
    (0..4).cartesian_product(0..maxs).map(|(d, t)| n[end_coord][d][t]).min().unwrap()
}

pub fn solve_1() -> i64 {
    solve(0, 3)
}

pub fn solve_2() -> i64 {
    solve(4, 10)
}
