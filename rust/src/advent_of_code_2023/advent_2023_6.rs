pub fn solve_1() -> usize {
    let times = [42, 89, 91, 89];
    let dist = [308, 1170, 1291, 1467];
    (0..4).map(|r| (0..times[r]).filter(|i| (times[r] - i) * i > dist[r]).count()).product()
}

pub fn solve_2() -> usize {
    (0..42899189i64).filter(|i| (42899189i64 - i) * i > 308117012911467i64).count()
}
