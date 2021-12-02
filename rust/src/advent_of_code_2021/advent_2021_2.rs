use crate::utils::read_lines;

pub fn solve_1() -> i64 {
    let lines = read_lines("advent_2021/2.txt");
    let mut depth = 0;
    let mut horizontal = 0;
    for line in lines {
        let s: Vec<_> = line.split(' ').collect();
        let v: i64 = s[1].parse().unwrap();
        match s[0] {
            "up" => depth -= v,
            "down" => depth += v,
            "forward" => horizontal += v,
            &_ => panic!(),
        }
    }
    horizontal * depth
}

pub fn solve_2() -> i64 {
    let lines = read_lines("advent_2021/2.txt");
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for line in lines {
        let s: Vec<_> = line.split(' ').collect();
        let v: i64 = s[1].parse().unwrap();
        match s[0] {
            "up" => aim -= v,
            "down" => aim += v,
            "forward" => {
                horizontal += v;
                depth += aim * v;
            }
            &_ => panic!(),
        }
    }
    horizontal * depth
}
