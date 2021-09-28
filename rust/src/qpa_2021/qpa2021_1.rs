use serde_json::{json, Map, Value};

fn count_ones(x: u64) -> u64 {
    let mut n = x;
    let mut count = 0;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

pub fn solve(v: &Value) -> Value {
    let nums: Vec<_> = v["set"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_u64().unwrap())
        .collect();
    let max_ones = nums.iter().map(|x| count_ones(*x)).max().unwrap_or(0);
    let insane_numbers: Vec<_> = nums
        .into_iter()
        .filter(|x| count_ones(*x) == max_ones)
        .collect();

    json!({ "insane_numbers": Value::from(insane_numbers) })
}
