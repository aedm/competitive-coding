use crate::utils::read_lines;
use itertools::Itertools;

pub fn solve_1() -> i64 {
    let mut s = 1i64;
    let mut t = 1;
    let mut v = vec![(s, t)];
    for line in read_lines("advent_2022/10.txt") {
        let p = line.split(' ').collect_vec();
        match p[0] {
            "addx" => {
                s += p[1].parse::<i64>().unwrap();
                t += 2;
                v.push((s, t));
            }
            "noop" => {
                t += 1;
                v.push((s, t));
            }
            _ => panic!(),
        }
    }
    let q = [20, 60, 100, 140, 180, 220];
    q.iter()
        .map(|x| {
            let k = v.iter().find_position(|(s, t)| t > x).unwrap();
            let k = v[k.0 - 1].0;
            k * x
        })
        .sum()
}

pub fn solve_2() -> () {
    let lines = read_lines("advent_2022/10.txt");
    let mut it = lines.iter();
    let mut pos = 1i64;
    let mut nextp = 1i64;
    let mut v = vec![String::new(); 6];
    let mut t = 0;
    for r in 0..6 {
        for c in 0..40 {
            if t == 0 {
                pos = nextp;
                let line = it.next().unwrap();
                let p = line.split(' ').collect_vec();
                match p[0] {
                    "addx" => {
                        nextp = pos + p[1].parse::<i64>().unwrap();
                        t = 2;
                    }
                    "noop" => {
                        nextp = pos;
                        t = 1;
                    }
                    _ => panic!(),
                }
            }
            t -= 1;
            let m = (pos - c).abs() <= 1;
            v[r].push(if m { 'X' } else { '.' });
        }
    }
    println!("{:#?}", v);
}
