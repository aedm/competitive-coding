pub fn solve() -> usize {
    const K: usize = 1_000_000;
    let sieve = primal::Sieve::new(K);
    let mut sum_divs: Vec<usize> =
        (1..K).map(|x| sum_divisors(&sieve.factor(x).unwrap(), 0, 1) - x).collect();
    sum_divs.insert(0, 0);

    let mut c = vec![0; K];
    let mut max_len = 0;
    let mut min_elem = 0;
    for i in 2..K {
        if c[i] != 0 {
            continue;
        }
        let mut x = i;
        while x >= i && x < K && c[x] == 0 {
            c[x] = i;
            x = sum_divs[x];
        }
        if x < i || x >= K || c[x] < i {
            continue;
        }
        let mut y = x;
        let mut min = x;
        let mut len = 1;
        loop {
            y = sum_divs[y];
            if y == x {
                break;
            }
            len += 1;
            if y < min {
                min = y;
            }
        }
        if len > max_len {
            max_len = len;
            min_elem = min;
        }
    }
    min_elem
}

fn sum_divisors(factors: &Vec<(usize, usize)>, c: usize, p: usize) -> usize {
    if c == factors.len() {
        return p;
    }
    let f = factors[c];
    (0..=f.1).map(|x| sum_divisors(factors, c + 1, p * f.0.pow(x as u32))).sum()
}
