pub fn solve2() -> String {
    let target = 2_000_000_i64;
    let (mut min, mut a, mut b) = (target, 0i64, 0i64);

    for x in 2..1500 {
        for y in 2..1500 {
            let area = x * (x + 1) * y * (y + 1) / 4;
            let d = (target - area).abs();
            if min > d {
                min = d;
                a = x;
                b = y;
            }
        }
    }
    println!("{:?} {} {}", min, a, b);
    format!("{}", a * b)
}

pub fn solve() -> String {
    let target = 2_000_000;
    let (mut min, mut xmin, mut ymin) = (target, 0, 0);

    for x in 1.. {
        if x * x + x > target {
            break;
        }
        let c = (4 * target) as f64 / (x * x + x) as f64;
        let y = ((-1.0 + (1.0 + 4.0 * c).sqrt()) / 2.0).floor() as i32;
        test_min(&mut min, &mut xmin, &mut ymin, x, y, target);
        test_min(&mut min, &mut xmin, &mut ymin, x, y + 1, target);
    }
    println!("{:?} {} {}", min, xmin, ymin);
    format!("{}", xmin * ymin)
}

fn test_min(min: &mut i32, a: &mut i32, b: &mut i32, x: i32, y: i32, target: i32) {
    let area = x * (x + 1) * y * (y + 1) / 4;
    let d = (target - area).abs();
    if *min > d {
        *min = d;
        *a = x;
        *b = y;
    }
}
