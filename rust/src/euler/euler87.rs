pub fn solve() -> i32 {
    const SIZE: usize = 50_000_000;
    let mut v = vec![false; SIZE];

    let max_prime = (SIZE as f64).sqrt().ceil() as usize;
    let mut primes: Vec<usize> = Vec::new();
    for i in 2..max_prime {
        if !primes.iter().any(|x| i % *x == 0) {
            primes.push(i);
        }
    }

    for a in &primes {
        let a2 = a * a;
        for b in &primes {
            let b3 = b * b * b + a2;
            if b3 >= SIZE {
                break;
            }
            for c in &primes {
                let c4 = c * c * c * c + b3;
                if c4 >= SIZE {
                    break;
                }
                v[c4] = true;
            }
        }
    }

    v.iter().filter(|x| **x).count() as i32
}
