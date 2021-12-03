use crate::utils::read_lines;

pub fn solve_1() -> i64 {
    let lines = read_lines("advent_2021/3.txt");
    let (mut gamma, mut epsilon) = (0, 0);
    for i in 0..lines[0].len() {
        let ones = lines.iter().filter(|x| x.as_bytes()[i] == '1' as u8).count();
        gamma = gamma * 2;
        epsilon = epsilon * 2;
        if ones > lines.len() - ones {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    gamma * epsilon
}

fn find_value(lines: &[String], flip: bool) -> i64 {
    let mut v: Vec<_> = (0..lines.len()).collect();
    for i in 0..lines[0].len() {
        let ones = v.iter().filter(|&&x| lines[x].as_bytes()[i] == '1' as u8).count();
        let more = if (ones >= v.len() - ones) ^ flip { '1' } else { '0' };
        v = v.iter().filter(|&&x| lines[x].as_bytes()[i] == more as u8).cloned().collect();
        if v.len() == 1 {
            return lines[v[0]].chars().fold(0, |acc, x| acc * 2 + (x as i64 - '0' as i64));
        }
    }
    panic!();
}

pub fn solve_2() -> i64 {
    let lines = read_lines("advent_2021/3.txt");
    let oxygen = find_value(&lines, false);
    let co2 = find_value(&lines, true);
    oxygen * co2
}
