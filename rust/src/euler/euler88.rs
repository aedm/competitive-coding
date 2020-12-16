use std::collections::BTreeSet;

const K_MAX: i32 = 12_000;

pub fn solve() -> i32 {
    let mut v = [K_MAX * 2 + 1; K_MAX as usize + 1];
    step(&mut v, 0, 1, 0, 2);
    v[2..]
        .iter()
        .map(|x| *x)
        .collect::<BTreeSet<i32>>()
        .iter()
        .sum()
}

fn step(v: &mut [i32], count: i32, prod: i32, sum: i32, low: i32) {
    let high = K_MAX * 2 / prod;
    for x in low..=high {
        let new_prod = prod * x;
        let new_sum = sum + x;
        let k = new_prod - new_sum + count + 1;
        if count > 0 && k <= K_MAX && v[k as usize] > new_prod {
            v[k as usize] = new_prod;
        }
        if new_prod < K_MAX {
            step(v, count + 1, new_prod, new_sum, x)
        }
    }
}
