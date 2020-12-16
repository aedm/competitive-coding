use crate::utils::read_lines;

pub fn solve() -> i32 {
    read_lines("euler/p089_roman.txt").iter().map(calc).sum()
}

fn calc(s2: &String) -> i32 {
    let s: Vec<char> = s2.chars().collect();
    let mut sum: usize = 0;
    let mut i = 0;
    while i < s.len() {
        let next = if i < s.len() - 1 { s[i + 1] } else { '.' };
        match (s[i], next) {
            ('I', 'V') => {
                sum += 4;
                i += 1;
            }
            ('I', 'X') => {
                sum += 9;
                i += 1
            }
            ('I', _) => sum += 1,
            ('V', _) => sum += 5,
            ('X', 'L') => {
                sum += 40;
                i += 1
            }
            ('X', 'C') => {
                sum += 90;
                i += 1
            }
            ('X', _) => sum += 10,
            ('L', _) => sum += 50,
            ('C', 'D') => {
                sum += 400;
                i += 1
            }
            ('C', 'M') => {
                sum += 900;
                i += 1
            }
            ('C', _) => sum += 100,
            ('D', _) => sum += 500,
            ('M', _) => sum += 1000,
            _ => panic!("oops"),
        }
        i += 1;
    }

    let min = [0, 1, 2, 3, 2, 1, 2, 3, 4, 2];
    let nc = (sum / 1000) + min[(sum % 1000) / 100] + min[(sum % 100) / 10] + min[sum % 10];

    (s2.len() - nc) as i32
}
