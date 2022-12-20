use crate::utils::read_lines;
use itertools::Itertools;

pub fn solve(key: isize, loops: usize) -> isize {
    let mut n = read_lines("advent_2022/20.txt")
        .into_iter()
        .map(|s| s.parse::<isize>().unwrap() * key)
        .collect_vec();
    let mut order = (0..n.len() as isize).collect_vec();
    let mut vp = n.iter().cloned().enumerate().map(|(i, v)| (i as isize, v)).collect_vec();
    for _ in 0..loops {
        for i in 0..n.len() {
            let mut p = order[i];
            let vpp = vp[p as usize];
            let t = ((p - 1) + vpp.1).rem_euclid(n.len() as isize - 1) + 1;
            let (l, d) = ((t - p).abs(), (t - p).signum());
            for i in 0..l {
                vp[(p + i * d) as usize] = vp[(p + (i + 1) * d) as usize];
                order[vp[(p + i * d) as usize].0 as usize] = p + i * d;
            }
            vp[(p + l * d) as usize] = vpp;
            order[vpp.0 as usize] = t;
        }
    }
    let np = vp.iter().find_position(|(_, v)| *v == 0).unwrap().0;
    [1000, 2000, 3000].iter().map(|i| vp[(np + i) % n.len()].1).sum::<isize>()
}

pub fn solve_1() -> isize {
    solve(1, 1)
}

pub fn solve_2() -> isize {
    solve(811589153, 10)
}
