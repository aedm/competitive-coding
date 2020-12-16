pub fn solve() -> i64 {
    let mut s = 0;
    for a in 2i64..(1_000_000_000 / 3) {
        check(a, a - 1, &mut s);
        check(a, a + 1, &mut s);
    }
    s
}

fn check(a: i64, b: i64, s: &mut i64) {
    let x = (2 * a + b) * (2 * a - b);
    let y = (x as f64).sqrt() as i64;
    let res = y * y == x && (b % 2 == 0 || y % 4 == 0);
    if res {
        *s += 2 * a + b;
    }
}
