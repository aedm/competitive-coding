use crate::utils::read_lines_split;
use itertools::Itertools;

fn count_ways(v: &[char], nums: &[usize]) -> usize {
    let mut m = vec![vec![0; v.len()]; nums.len() + 1];
    m[0][0] = 1;
    for i in 1..v.len() {
        m[0][i] = if v[i] == '#' { 0 } else { m[0][i - 1] };
    }
    for (y, n) in nums.iter().enumerate() {
        for i in n + 1..v.len() {
            if v[i] == '.' {
                m[y + 1][i] = m[y + 1][i - 1]
            } else {
                if v[i - n + 1..=i].iter().all(|&c| c == '#' || c == '?') && v[i - n] != '#' {
                    m[y + 1][i] = m[y][i - n - 1]
                }
                if v[i] == '?' {
                    m[y + 1][i] += m[y + 1][i - 1]
                }
            }
        }
    }
    m[nums.len()][v.len() - 1]
}

pub fn solve() -> usize {
    read_lines_split("advent_2023/12.txt", &[' ', ','])
        .iter()
        .map(|l| {
            let nums = l[1..].iter().map(|s| s.parse::<usize>().unwrap()).collect_vec();
            count_ways(&format!("..{}", &l[0]).chars().collect_vec(), &nums)
        })
        .sum::<usize>()
}

pub fn solve_2() -> usize {
    read_lines_split("advent_2023/12.txt", &[' ', ','])
        .iter()
        .map(|l| {
            let nums = l[1..].iter().map(|s| s.parse::<usize>().unwrap()).collect_vec();
            let nums = nums.iter().cycle().take(nums.len() * 5).cloned().collect_vec();
            let k = format!("..{}?{}?{}?{}?{}", &l[0], &l[0], &l[0], &l[0], &l[0]);
            count_ways(&k.chars().collect_vec(), &nums)
        })
        .sum::<usize>()
}
