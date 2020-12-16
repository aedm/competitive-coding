use crate::utils::read_lines;

pub fn solve_2() -> i64 {
    let lines = read_lines("advent_2020/1.txt");
    let mut nums: Vec<_> = lines.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    for a in 0..nums.len() {
        for b in a..nums.len() {
            for c in a..nums.len() {
                if nums[a] + nums[b] + nums[c] == 2020 {
                    return nums[a] * nums[b] * nums[c];
                }
            }
        }
    }
    -1
}

pub fn solve_1() -> i64 {
    let lines = read_lines("advent_2020/1.txt");
    let mut nums: Vec<_> = lines.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    for a in 0..nums.len() {
        for b in a..nums.len() {
            if nums[a] + nums[b] == 2020 {
                return nums[a] * nums[b];
            }
        }
    }
    -1
}
