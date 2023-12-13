use crate::utils::read_lines;
use itertools::Itertools;

fn solve_c(factor: i64) -> usize {
    let l = read_lines("advent_2023/11.txt").iter().map(|l| l.chars().collect_vec()).collect_vec();
    let (w, h) = (l[0].len(), l.len());
    let s = (0..w)
        .cartesian_product(0..h)
        .filter(|&(x, y)| l[y as usize][x as usize] == '#')
        .collect_vec();
    let ex = (0..w).filter(|&x| s.iter().all(|s| s.0 != x)).collect_vec();
    let ey = (0..h).filter(|&y| s.iter().all(|s| s.1 != y)).collect_vec();
    (0..s.len())
        .tuple_combinations()
        .map(|(a, b)| {
            let (y1, y2) = if s[a].1 < s[b].1 { (s[a].1, s[b].1) } else { (s[b].1, s[a].1) };
            let lx = ex.partition_point(|x| s[b].0 > *x) - ex.partition_point(|x| s[a].0 > *x);
            let ly = ey.partition_point(|y| y2 > *y) - ey.partition_point(|y| y1 > *y);
            (s[b].0 - s[a].0) + (y2 - y1) + (lx + ly) * (factor as usize - 1)
        })
        .sum::<usize>()
}

pub fn solve_1() -> usize {
    solve_c(2)
}

pub fn solve() -> usize {
    solve_c(1000000)
}
