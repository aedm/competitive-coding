use crate::utils::read_lines;

pub fn solve_1() -> i64 {
    let lines = read_lines("advent_2021/7.txt");
    let mut cr: Vec<i64> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();
    cr.sort();
    let median = cr[cr.len() / 2];
    cr.iter().map(|x| (x - median).abs()).sum()
}

pub fn solve_2() -> Option<i64> {
    let lines = read_lines("advent_2021/7.txt");
    let mut cr: Vec<i64> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();
    (*cr.iter().min()?..=*cr.iter().max()?)
        .map(|x| (cr.iter().map(|c| (x - c).abs() * ((x - c).abs() + 1) / 2)).sum())
        .min()
}
