pub fn solve() -> String {
    let mut solutions = 0;
    for m in 1.. {
        for x in 1..=m {
            for y in x..=m {
                let xd = (x + m) * (x + m) + y * y;
                let yd = (y + m) * (y + m) + x * x;
                let md = (y + x) * (y + x) + m * m;
                let minxy = if xd < yd { xd } else { yd };
                let min = if minxy < md { minxy } else { md };
                let root = (min as f64).sqrt() as i32;
                if root * root == min {
                    solutions += 1;
                }
                if solutions > 1_000_000 {
                    return format!("{}", m);
                }
            }
        }
    }
    "".to_string()
}
