pub fn solve() -> usize {
    let mut n1 = [0usize; 6];
    cube1(&mut n1, 0, 0)
}

fn cube1(n1: &mut [usize], c: usize, min: usize) -> usize {
    let mut solutions = 0;
    for i in min..=(4 + c) {
        n1[c] = if i == 9 { 6 } else { i };
        if c < 5 {
            solutions += cube1(n1, c + 1, i + 1);
        } else {
            let mut n2 = [0usize; 6];
            solutions += cube2(n1, &mut n2, 0, 0);
        }
    }
    solutions
}

fn cube2(n1: &[usize], n2: &mut [usize], c: usize, min: usize) -> usize {
    let mut solutions = 0;
    for i in min..=(4 + c) {
        n2[c] = if i == 9 { 6 } else { i };
        if c < 5 {
            solutions += cube2(n1, n2, c + 1, i + 1);
        } else {
            if check(n1, n2) {
                solutions += 1
            }
        }
    }
    solutions
}

fn check(n1: &[usize], n2: &[usize]) -> bool {
    let s1 = n1.iter().fold(0usize, |acc, x| acc * 10 + *x);
    let s2 = n2.iter().fold(0usize, |acc, x| acc * 10 + *x);
    if s1 > s2 {
        return false;
    }
    let mut b1 = [false; 10];
    let mut b2 = [false; 10];
    n1.iter().for_each(|x| b1[*x] = true);
    n2.iter().for_each(|x| b2[*x] = true);
    let p = |a: usize, b: usize| (b1[a] && b2[b]) || (b1[b] && b2[a]);
    p(0, 1) && p(0, 4) && p(0, 6) && p(1, 6) && p(2, 5) && p(3, 6) && p(4, 6) && p(6, 4) && p(8, 1)
}
