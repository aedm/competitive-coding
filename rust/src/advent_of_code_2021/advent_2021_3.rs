use crate::utils::read_lines;

pub fn solve_1() -> i64 {
    let lines = read_lines("advent_2021/3.txt");
    let (mut gamma, mut epsilon) = (0, 0);
    for i in 0..lines[0].len() {
        let ones = lines
            .iter()
            .filter(|x| x.as_bytes()[i] == '1' as u8)
            .count();
        let d = (ones > lines.len() - ones) as i64;
        gamma = gamma * 2 + d;
        epsilon = epsilon * 2 + 1 - d;
    }
    gamma * epsilon
}

fn find_value(lines: &[String], flip: bool) -> i64 {
    let mut v: Vec<_> = (0..lines.len()).collect();
    for i in 0..lines[0].len() {
        let ones = v
            .iter()
            .filter(|&&x| lines[x].as_bytes()[i] == '1' as u8)
            .count();
        let more = '0' as u8 + ((ones >= v.len() - ones) ^ flip) as u8;
        v = v
            .into_iter()
            .filter(|&x| lines[x].as_bytes()[i] == more)
            .collect();
        if v.len() == 1 {
            return i64::from_str_radix(&lines[v[0]], 2).unwrap();
        }
    }
    panic!();
}

pub fn solve_2() -> i64 {
    let lines: Vec<_> = read_lines("advent_2021/3.txt");
    let oxygen = find_value(&lines, false);
    let co2 = find_value(&lines, true);
    oxygen * co2
}
