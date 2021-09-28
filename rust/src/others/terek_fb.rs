use std::cmp::{max, min};

pub fn solve_1(matrix: &[Vec<i32>]) {
    if matrix.len() == 0 {
        return;
    }
    let width = matrix[0].len();
    let max_i = matrix.len() + width - 2;
    for i in 0..=max_i {
        let min_y = max(width, i + 1) - width;
        let max_y = min(i, matrix.len() - 1);
        for y in min_y..=max_y {
            print!("{} ", matrix[y][i - y]);
        }
        println!();
    }
}

fn scan(s: &str, c: char) -> usize {
    s.chars().take_while(|x| *x == c).count()
}

pub fn solve_2b(days: &[char], free: i64) {
    let (mut available, mut solution, mut start) = (free, 0, 0);
    for (index, day) in days.iter().enumerate() {
        available -= (*day == 'W') as i64;
        while available < 0 {
            available += (days[start] == 'W') as i64;
            start += 1;
        }
        solution = max(solution, index - start + 1);
    }
    println!("Maximum vacation is {} days long.", solution);
}

pub fn solve_2(days: &str, free: isize) {
    let mut weeks = vec![];
    let mut q = days;
    while q.len() > 0 {
        let weekend = scan(q, 'H');
        let work_day = scan(&q[weekend..], 'W');
        q = &q[(weekend + work_day)..];
        weeks.push((weekend as isize, work_day as isize));
    }

    let mut until_week = 0;
    let mut work_days_taken = 0;
    let mut days_between = 0;
    let mut vacation_max = 0;
    for first_week in 0..weeks.len() {
        while until_week < weeks.len() {
            let next = weeks[until_week];
            if work_days_taken + next.1 > free {
                break;
            }
            work_days_taken += next.1;
            days_between += next.1 + next.0;
            until_week += 1;
        }
        let mut days_before = if first_week == 0 {
            0
        } else {
            weeks[first_week - 1].1
        };
        let days_after = if until_week < weeks.len() {
            weeks[until_week].1
        } else {
            0
        };
        let next_weekend = if until_week < weeks.len() {
            weeks[until_week].0
        } else {
            0
        };
        let plus_days = min(days_before + days_after, free - work_days_taken);
        let vacation_length = days_between + plus_days + next_weekend;
        vacation_max = max(vacation_max, vacation_length);

        let current = weeks[first_week];
        days_between -= current.1 + current.0;
        work_days_taken -= current.1;
    }

    println!("Maximum vacation is {} days long.", vacation_max);
}

pub fn main() {
    let k: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![1, 2, 3]];
    solve_1(&k);

    let days = "WWHHWWHHHHWWWHHHHWHWWWWWW";
    solve_2(&days, 1);
    solve_2b(&days.chars().collect::<Vec<_>>(), 1);
}
