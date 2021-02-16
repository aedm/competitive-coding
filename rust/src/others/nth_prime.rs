pub fn main() {
    let mut p = vec![];
    for i in 2u64.. {
        if p.iter().all(|x| i % x > 0) {
            p.push(i);
            if p.len() == 1_000_000_000_000 {
                println!("{:x}", i);
                break;
            }
        }
    }
}