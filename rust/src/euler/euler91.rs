pub fn solve() -> i32 {
    let mut s = 0;
    let k = 50;
    for x1 in 0..=k {
        for y1 in 0..=k {
            for x2 in x1..=k {
                for y2 in 0..=k {
                    let a = x1 * x1 + y1 * y1;
                    let b = x2 * x2 + y2 * y2;
                    let c = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
                    if (x1 != x2 || y1 != y2)
                        && (x2 > x1 || y2 > y1)
                        && (x1 > 0 || y1 > 0)
                        && (x2 > 0 || y2 > 0)
                        && (a + b == c || a + c == b || b + c == a)
                    {
                        s += 1;
                    }
                }
            }
        }
    }
    s
}
