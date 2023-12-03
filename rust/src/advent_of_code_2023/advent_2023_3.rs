use crate::utils::read_lines;
use itertools::Itertools;
use regex::Regex;
use std::fmt::{Debug, Display};

fn read_maps() -> (Vec<Vec<char>>, Vec<(i32, i32, i32, i32)>) {
    let num_pattern = Regex::new(r"(\d+)").unwrap();

    let lines = read_lines("advent_2023/3.txt");
    let c = lines.iter().map(|l| l.chars().collect_vec()).collect_vec();
    let nums = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            num_pattern.find_iter(l).map(move |m| {
                (
                    m.start() as i32,
                    y as i32,
                    m.as_str().len() as i32,
                    m.as_str().parse::<i32>().unwrap(),
                )
            })
        })
        .collect_vec();
    (c, nums)
}

pub fn solve_1() -> i32 {
    let (c, nums) = read_maps();
    nums.iter()
        .filter(|(x, y, l, v)| {
            (-1..=*l).cartesian_product(-1..=1).any(|(dx, dy)| {
                let (mx, my) = (x + dx, y + dy);
                if mx > 0 && my > 0 && mx < c[0].len() as i32 && my < c.len() as i32 {
                    let mv = c[my as usize][mx as usize];
                    !mv.is_numeric() && mv != '.'
                } else {
                    false
                }
            })
        })
        .map(|(_, _, _, v)| *v)
        .sum::<i32>()
}

pub fn solve() -> i32 {
    let (c, nums) = read_maps();
    let mut gears = vec![vec![(0, 1); c[0].len()]; c.len()];
    for (x, y, l, v) in &nums {
        for (dx, dy) in (-1..=*l).cartesian_product(-1..=1) {
            let (mx, my) = (x + dx, y + dy);
            if mx > 0
                && my > 0
                && mx < c[0].len() as i32
                && my < c.len() as i32
                && c[my as usize][mx as usize] == '*'
            {
                let g = &mut gears[my as usize][mx as usize];
                *g = (g.0 + 1, g.1 * *v)
            }
        }
    }
    gears.iter().flatten().filter_map(|e| (e.0 == 2).then(|| e.1)).sum::<i32>()
}
