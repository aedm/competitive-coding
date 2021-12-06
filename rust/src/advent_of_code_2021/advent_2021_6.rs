use crate::utils::read_lines;

fn solve(days: i64) -> i64 {
    let lines = read_lines("advent_2021/6.txt");
    let mut nums: Vec<i64> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();
    let mut k = vec![0; 11];
    nums.iter().for_each(|&x| k[x as usize] += 1);
    for i in 0..days {
        k[9] += k[0];
        k[7] += k[0];
        (1..11).for_each(|o| k[o - 1] = k[o]);
    }
    k.iter().sum()
}

pub fn solve_1() -> i64 {
    solve(80)
}

pub fn solve_2() -> i64 {
    solve(256)
}
