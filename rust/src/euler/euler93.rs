use permutator::Permutation;

type F = fraction::Fraction;

pub fn solve() -> String {
    let ops: Vec<fn(x: &F, y: &F) -> F> =
        vec![|x, y| x + y, |x, y| x - y, |x, y| x * y, |x, y| x / y];

    let mut max = 0;
    let mut maxn = (0, 0, 0, 0);

    let num_orders: Vec<_> = vec![0, 1, 2, 3].permutation().collect();
    for a in 1..10 {
        let af = F::from(a);
        for b in (a + 1)..10 {
            let bf = F::from(b);
            for c in (b + 1)..10 {
                let cf = F::from(c);
                for d in (c + 1)..10 {
                    let df = F::from(d);

                    let x = [&af, &bf, &cf, &df];
                    let mut nums = [false; 10000];
                    let mut app = |f: &F| {
                        if f.is_normal() && *f.denom().unwrap() == 1 && f.is_sign_positive() {
                            // println!("{:?}", f);
                            nums[*f.numer().unwrap() as usize] = true;
                        }
                    };

                    for o1 in 0..4 {
                        let op1 = ops[o1];
                        for o2 in 0..4 {
                            let op2 = ops[o2];
                            for o3 in 0..4 {
                                let op3 = ops[o3];
                                num_orders.iter().for_each(|order| {
                                    let n1 = x[order[0]];
                                    let n2 = x[order[1]];
                                    let n3 = x[order[2]];
                                    let n4 = x[order[3]];
                                    app(&op2(&op1(n1, n2), &op3(n3, n4)));
                                    app(&op3(&op2(&op1(n1, n2), n3), n4));
                                    app(&op3(&op1(n1, &op2(n2, n3)), n4));
                                    app(&op1(n1, &op3(&op2(n2, n3), n4)));
                                    app(&op1(n1, &op2(n2, &op3(n3, n4))));
                                });
                            }
                        }
                    }

                    let count = nums[1..].iter().take_while(|x| **x).count();
                    if count > max {
                        max = count;
                        maxn = (a, b, c, d);
                    }
                }
            }
        }
    }
    format!("{}{}{}{}", maxn.0, maxn.1, maxn.2, maxn.3)
}
