use std::io::stdin;
use std::cmp::{max, min};

const MAX_SUM: u64 = 10_000_000_000_000_000u64;
const MAX_BITS: usize = 62;

pub fn main() {
    solve().iter().for_each(|x| println!("{}", x));
}

pub fn solve() -> Vec<String> {
    let mut s = String::new();
    let _ = stdin().read_line(&mut s);
    let count = s.trim().parse::<usize>().unwrap();
    let inputs = (0..count).map(|_| {
        let mut s = String::new();
        let _ = stdin().read_line(&mut s);
        s.trim_end().parse::<u64>().unwrap()
    }).collect::<Vec<_>>();
    solve_all(&inputs)
}

fn solve_all(inputs: &Vec<u64>) -> Vec<String> {
    let mut m = Vec::<[u64; MAX_BITS]>::new();
    let mut sums= Vec::<u64>::new();
    let mut sum = 0;
    for i in 0.. {
        m.push([0u64; MAX_BITS]);
        for bits in 0..MAX_BITS {
            m[i][bits] = match (i, bits) {
                (0, _) => 1,
                (_, 0) => 0,
                (s, b) => {
                    let exp = 1 << (b - 1);
                    (0..=min(s, 9*exp)).step_by(exp).map(|it| m[s - it][b - 1]).sum()
                }
            }
        }
        sum += m[i][MAX_BITS - 1];
        sums.push(sum);
        if sum > MAX_SUM {
            break;
        }
    }
    m[0][0] = 0;

    inputs.iter().map(|x| find(*x, &m, &sums)).collect()
}

fn find(input: u64, m: &[[u64; MAX_BITS]], sums: &[u64]) -> String {
    if input == 1 { return "0".to_string(); }

    let v = sums.binary_search(&input);
    let mut bit_sum: u64 = match v {
        Ok(x) => x,
        Err(x) => x,
    } as u64;
    let mut x = input - sums[bit_sum as usize-1];
    let mut result = vec![];
    for bit in (1..MAX_BITS).rev() {
        let pot = 1 << (bit - 1);
        let mut dec = 0u64;
        let mut count = 0;
        while bit_sum >= (dec + 1) * pot {
            let p = m[(bit_sum - dec * pot) as usize][bit - 1];
            if x <= count + p { break; }
            count += p;
            dec += 1;
        }
        x -= count;
        bit_sum -= dec * pot;
        result.push(dec);
    }
    result.iter().skip_while(|x| **x == 0).map(|x| x.to_string()).collect()
}