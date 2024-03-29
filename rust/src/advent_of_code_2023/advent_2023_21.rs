use crate::utils::array2d::{v, Map2D};
use crate::utils::read_lines;
use itertools::Itertools;

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
        let mut big_map = Map2D::from_fn(m.w * 2 + 1, m.h * 2 + 1, |p| {
            m[(s + dir * p + m.size() * 2) % m.size()]
        });
        let mut distance = Map2D::from_map(&big_map, |_| None);
        distance[v(0, 0)] = Some(0);
        distance.flood4(vec![v(0, 0)], |c, a, prev| {
            (big_map[c] == '.' && a.is_none()).then(|| Some(prev.unwrap() + 1))
        });
        for (x, y) in (0..big_map.w - 1).cartesian_product(0..big_map.h - 1) {
            if (x + y) % 2 == steps % 2 {
                if let Some(c) = distance[v(x, y)] {
                    let a = (steps - c).div_euclid(big_map.w - 1);
                    acc += (a + 1) * (a + 2) / 2;
                }
            }
        }
    }
    acc - (steps + 1) * 2 - (steps + 1) % 2
}
