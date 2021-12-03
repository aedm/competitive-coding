use crate::utils::read_lines;

pub fn solve_1() -> usize {
    let lines = read_lines("advent_2021/1.txt");
    let nums: Vec<i64> = lines.iter().map(|x| x.parse().unwrap()).collect();
    nums[1..]
        .iter()
        .zip(nums.iter())
        .filter(|(x, y)| x > y)
        .count()
}

pub fn solve_2() -> usize {
    let lines = read_lines("advent_2021/1.txt");
    let nums: Vec<i64> = lines.iter().map(|x| x.parse().unwrap()).collect();
    nums[3..]
        .iter()
        .zip(nums.iter())
        .filter(|(x, y)| x > y)
        .count()
}
