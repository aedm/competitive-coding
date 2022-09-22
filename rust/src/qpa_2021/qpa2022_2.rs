use serde_json::{json, Map, Value};

pub fn solve(v: &Value) -> Value {
    println!("{:?}", v);
    let x = v.as_array().unwrap();
    let y = x
        .into_iter()
        .map(|x| x.as_u64().unwrap())
        .collect::<Vec<_>>();
    println!("{:?}", y);

    let mut v1 = vec![0; y.len()];
    let mut v2 = vec![0; y.len()];
    let mut min = 0;
    let mut last = 0;
    for i in 0..y.len() {
        if y[i] > last {
            min += 1;
        } else {
            min = 1;
        }
        v1[i] = min;
        last = y[i];
    }

    let mut min = 0;
    let mut last = 0;
    for i in 0..y.len() {
        let i = y.len() - i - 1;
        if y[i] > last {
            min += 1;
        } else {
            min = 1;
        }
        v2[i] = min;
        last = y[i];
    }

    for i in 0..y.len() {
        v1[i] = v1[i].max(v2[i]);
    }
    let sol = v1.iter().sum::<i64>();

    json!(sol)
}
