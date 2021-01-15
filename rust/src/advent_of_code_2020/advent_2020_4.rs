use crate::utils::read_lines;
use regex::Regex;
use std::collections::BTreeSet;

pub fn solve_1() -> usize {
    let ids = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut lines = read_lines("advent_2020/4.txt");
    lines.push("".into());
    let mut keys: BTreeSet<&str> = BTreeSet::new();
    let mut count = 0;
    for line in &lines {
        if line.len() == 0 {
            if ids.iter().all(|id| keys.contains(id)) {
                count += 1;
            }
            keys.clear();
        } else {
            let pairs: Vec<_> = line.trim().split(" ").collect();
            for pair in &pairs {
                let l = pair.split(':').nth(0).unwrap();
                keys.insert(l);
            }
        }
    }
    count
}

pub fn solve_2() -> usize {
    let hair_pattern: Regex =
        Regex::new(r"^#[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]$").unwrap();
    let ids = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut lines = read_lines("advent_2020/4.txt");
    lines.push("".into());
    let mut keys: BTreeSet<&str> = BTreeSet::new();
    let mut count = 0;
    for line in &lines {
        let trimmed = line.trim();
        if trimmed.len() == 0 {
            if ids.iter().all(|id| keys.contains(id)) {
                count += 1;
            }
            keys.clear();
        } else {
            let pairs: Vec<_> = trimmed.split(" ").collect();
            for pair in &pairs {
                let parts: Vec<_> = pair.split(':').collect();
                let key = parts[0];
                let value = parts[1];
                let is_valid = match key {
                    "byr" => {
                        let d = value.parse::<usize>().unwrap_or(0);
                        1920 <= d && 2002 >= d
                    }
                    "iyr" => {
                        let d = value.parse::<usize>().unwrap_or(0);
                        2010 <= d && 2020 >= d
                    }
                    "eyr" => {
                        let d = value.parse::<usize>().unwrap_or(0);
                        2020 <= d && 2030 >= d
                    }
                    "hgt" => {
                        if value.ends_with("cm") {
                            let d = value.trim_end_matches("cm").parse::<usize>().unwrap_or(0);
                            150 <= d && 193 >= d
                        } else if value.ends_with("in") {
                            let d = value.trim_end_matches("in").parse::<usize>().unwrap_or(0);
                            59 <= d && 76 >= d
                        } else {
                            false
                        }
                    }
                    "hcl" => hair_pattern.is_match(value),
                    "ecl" => {
                        let h = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                        h.contains(&value)
                    }
                    "pid" => value.len() == 9 && value.chars().all(|c| c >= '0' && c <= '9'),
                    _ => false,
                };
                if is_valid {
                    keys.insert(key);
                }
            }
        }
    }
    count
}
