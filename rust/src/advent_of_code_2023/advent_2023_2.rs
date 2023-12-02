use crate::utils::read_lines;
use itertools::Itertools;
use regex::Regex;

fn read_games() -> Vec<Vec<Vec<(u32, usize)>>> {
    const COLORS: [&str; 3] = ["red", "green", "blue"];
    let line_pattern: Regex = Regex::new(r"^Game (\d+): .*$").unwrap();
    read_lines("advent_2023/2.txt")
        .iter()
        .map(|l| {
            l.split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .map(|p| {
                    p.split(", ")
                        .map(|s| {
                            let d = s.split(" ").collect_vec();
                            let color = COLORS.iter().find_position(|&&c| c == d[1]).unwrap().0;
                            (d[0].parse::<u32>().unwrap(), color)
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec()
}

pub fn solve_1() -> usize {
    let games = read_games();
    let hand = [12, 13, 14];
    games
        .iter()
        .enumerate()
        .filter(|(i, g)| g.iter().all(|h| h.iter().all(|(count, dice)| hand[*dice] >= *count)))
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn solve_2() -> u32 {
    let games = read_games();
    let hand = [12, 13, 14];
    games
        .iter()
        .map(|g| {
            let mut mins = [0, 0, 0];
            for h in g {
                for (count, dice) in h {
                    mins[*dice] = mins[*dice].max(*count);
                }
            }
            mins[0] * mins[1] * mins[2]
        })
        .sum()
}
