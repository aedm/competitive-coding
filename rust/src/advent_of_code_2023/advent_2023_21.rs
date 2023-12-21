use crate::utils::array2d::{v, Map2D, DIRS4};
use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::VecDeque;

pub fn solve_1() -> i64 {
    solve(64)
}

pub fn solve_2() -> i64 {
    solve(26501365)
}

pub fn solve(steps: i64) -> i64 {
    let mut m = Map2D::from_text(&read_lines("advent_2023/21.txt"));
    let s = m.filter(|_, v| *v == 'S').map(|(p, _)| p).next().unwrap();
    m[s] = '.';

    let mut acc = 0;
    for dir in [v(1, 1), v(-1, 1), v(1, -1), v(-1, -1)] {
        let mut m2 = Map2D::new(m.w * 2 + 1, m.h * 2 + 1, '.');
        for (x, y) in (0..m2.w).cartesian_product(0..m2.h) {
            let target = v(x, y);
            let c = s + dir * target;
            let source = v((c.x + m.w * 2) % m.w, (c.y + m.w * 2) % m.h);
            m2[target] = m[source];
        }

        let start = v(0, 0);
        let mut distance = Map2D::from_map(&m2, |_| None);
        distance[start] = Some(0i64);
        let mut queue = VecDeque::from([start]);
        while let Some(c) = queue.pop_front() {
            for d in 0..4 {
                if let Some((nc, '.')) = m2.add_coord(c, DIRS4[d]) {
                    if distance[nc].is_none() {
                        distance[nc] = Some(distance[c].unwrap() + 1);
                        queue.push_back(nc);
                    }
                }
            }
        }

        for (x, y) in (0..m2.w - 1).cartesian_product(0..m2.h - 1) {
            if (x + y) % 2 == steps % 2 {
                if let Some(c) = distance[v(x, y)] {
                    let a = (steps - c).div_euclid(m2.w - 1);
                    acc += (a + 1) * (a + 2) / 2;
                }
            }
        }
    }
    acc - 4 * ((steps + 1) / 2) - 3 * ((steps + 1) % 2)
}
