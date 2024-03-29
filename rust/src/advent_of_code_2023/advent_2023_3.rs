use crate::utils::{map_add, read_lines};
use itertools::Itertools;
use regex::Regex;
use std::fmt::{Debug, Display};

fn read_maps() -> (Vec<Vec<char>>, Vec<(i64, i64, i64, i64)>) {
    let num_pattern = Regex::new(r"(\d+)").unwrap();
    let lines = read_lines("advent_2023/3.txt");
    let c = lines.iter().map(|l| l.chars().collect_vec()).collect_vec();
    let nums = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            num_pattern.find_iter(l).map(move |m| {
                (
                    m.start() as i64,
                    y as i64,
                    m.as_str().len() as i64,
                    m.as_str().parse::<i64>().unwrap(),
                )
            })
        })
        .collect_vec();
    (c, nums)
}

pub fn solve_1() -> i64 {
    let (c, nums) = read_maps();
    nums.iter()
        .filter(|(x, y, l, v)| {
            (-1..=*l).cartesian_product(-1..=1).any(|(dx, dy)| {
                if let Some((mx, my)) = map_add((*x, *y), (dx, dy), c[0].len() as i64, c.len() as i64) {
                    let mv = c[my as usize][mx as usize];
                    return !mv.is_numeric() && mv != '.';
                }
                false
            })
        })
        .map(|(_, _, _, v)| *v)
        .sum::<i64>()
}

pub fn solve_2() -> i64 {
    let (c, nums) = read_maps();
    let mut gears = vec![vec![(0, 1); c[0].len()]; c.len()];
    for (x, y, l, v) in &nums {
        for (dx, dy) in (-1..=*l).cartesian_product(-1..=1) {
            if let Some((mx, my)) = map_add((*x, *y), (dx, dy), c[0].len() as i64, c.len() as i64) {
                if c[my as usize][mx as usize] == '*' {
                    let g = &mut gears[my as usize][mx as usize];
                    *g = (g.0 + 1, g.1 * v)
                }
            }
        }
    }
    gears.iter().flatten().filter_map(|e| (e.0 == 2).then(|| e.1)).sum::<i64>()
}
