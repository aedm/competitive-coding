use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::HashMap;

pub fn solve_d(cycles: usize) -> i64 {
    let mut m =
        read_lines("advent_2023/14.txt").into_iter().map(|l| l.chars().collect_vec()).collect_vec();
    let (w, h) = (m[0].len() as i64, m.len() as i64);

    let ds = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut index = 0;
    let mut map_to_index = HashMap::from([(m.clone(), index)]);
    let mut index_to_map = HashMap::from([(index, m)]);
    let mut next_index = HashMap::<(usize, usize), usize>::new();

    for c in 0..cycles {
        let d = c % 4;
        if let Some(ni) = next_index.get(&(index, d)) {
            index = *ni;
            continue;
        }
        let mut m = index_to_map.get(&index).unwrap().clone();
        let mut rocks = (0i64..w)
            .cartesian_product(0i64..h)
            .filter(|&(x, y)| m[y as usize][x as usize] == 'O')
            .collect_vec();
        rocks.sort_by_key(|&(x, y)| (x as i64 * -ds[d].0, y as i64 * -ds[d].1));
        for start in rocks {
            let mut r = start;
            loop {
                let nr = (r.0 as i64 + ds[d].0, r.1 as i64 + ds[d].1);
                if !(nr.0 >= 0 && nr.0 < w && nr.1 >= 0 && nr.1 < h)
                    || m[nr.1 as usize][nr.0 as usize] != '.'
                {
                    break;
                }
                r = nr;
            }
            m[start.1 as usize][start.0 as usize] = '.';
            m[r.1 as usize][r.0 as usize] = 'O';
        }
        if let Some(ni) = map_to_index.get(&m) {
            next_index.insert((index, d), *ni);
            index = *ni;
        } else {
            let ni = index_to_map.len();
            map_to_index.insert(m.clone(), ni);
            index_to_map.insert(ni, m);
            next_index.insert((index, d), ni);
            index = ni;
        }
    }
    let m = index_to_map.get(&index).unwrap();
    (0..w)
        .cartesian_product(0..h)
        .filter(|&(x, y)| m[y as usize][x as usize] == 'O')
        .map(|(x, y)| h - y)
        .sum::<i64>()
}

pub fn solve() -> i64 {
    solve_d(1)
}

pub fn solve_2() -> i64 {
    solve_d(4_000_000_000)
}
