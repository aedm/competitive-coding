pub fn solve() -> i32 {
    let mut s = 0;
    for i in 1..10_000_000 {
        let mut k = i;
        while k != 1 && k != 89 {
            let mut d = 0;
            while k > 0 {
                d += (k % 10) * (k % 10);
                k /= 10;
            }
            k = d;
        }
        if k == 89 {
            s += 1
        }
    }
    s
}
