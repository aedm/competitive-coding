use crate::utils::read_lines;

pub fn solve_1() -> i64 {
    let lines = read_lines("advent_2021/1.txt");
    let nums: Vec<_> = lines
        .into_iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut x = 0;
    for i in 1..nums.len() {
        if nums[i - 1] < nums[i] {
            x += 1
        }
    }
    x
}

pub fn solve_2() -> i64 {
    let lines = read_lines("advent_2021/1.txt");
    let nums: Vec<_> = lines
        .into_iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut x = 0;
    for i in 3..nums.len() {
        if nums[i - 3] < nums[i] {
            x += 1
        }
    }
    x
}
