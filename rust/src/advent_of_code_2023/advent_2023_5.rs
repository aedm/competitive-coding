use crate::utils::read_lines;
use itertools::Itertools;
use std::fmt::Debug;

fn read_input() -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>) {
    let lines = read_lines("advent_2023/5.txt");
    let blocks = lines.split(|l| l.is_empty()).collect_vec();
    let seeds = blocks[0][0].split(' ').skip(1).map(|s| s.parse().unwrap()).collect_vec();
    let maps = blocks[1..]
        .iter()
        .map(|b| {
            b[1..]
                .iter()
                .map(|l| l.split(' ').filter_map(|s| s.parse().ok()).collect_tuple().unwrap())
                .collect_vec()
        })
        .collect_vec();
    (seeds, maps)
}

fn solve(maps: &[Vec<(i64, i64, i64)>], seeds: &[(i64, i64)]) -> i64 {
    let mut seeds = seeds.to_vec();
    for map in maps {
        let mut next_seeds = vec![];
        while let Some(s) = seeds.pop() {
            if let Some(m) = map.iter().find(|m| !(m.1 >= s.1 || s.0 >= m.1 + m.2)) {
                let start = s.0.clamp(m.1, m.1 + m.2);
                let end = s.1.clamp(m.1, m.1 + m.2);
                next_seeds.push((start + m.0 - m.1, end + m.0 - m.1));
                if s.0 < start {
                    seeds.push((s.0, start));
                }
                if s.1 > end {
                    seeds.push((end, s.1));
                }
            } else {
                next_seeds.push((s.0, s.1));
            }
        }
        seeds = next_seeds;
    }
    seeds.iter().map(|s| s.0).min().unwrap()
}

pub fn solve_1() -> i64 {
    let (seeds, maps) = read_input();
    let seeds = seeds.iter().map(|c| (*c, c + 1)).collect_vec();
    solve(&maps, &seeds)
}

pub fn solve_2() -> i64 {
    let (seeds, maps) = read_input();
    let seeds = seeds[..].chunks(2).map(|c| (c[0], c[0] + c[1])).collect_vec();
    solve(&maps, &seeds)
}
