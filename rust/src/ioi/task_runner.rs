use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;
use std::{fs, io};

type Solver = fn(&Vec<String>) -> Vec<String>;

pub fn run(input_folder: &str, solver: Solver) -> io::Result<()> {
    for entry in fs::read_dir(input_folder)? {
        let entry = entry?;
        let input_path = entry.path();
        if !input_path.is_dir() && input_path.extension().and_then(OsStr::to_str) == Some("in") {
            println!("Testing {:?}...", input_path);
            let input = read_as_string(&input_path)[1..].to_vec();

            let now = Instant::now();
            let solution = solver(&input);
            let elapsed = now.elapsed().as_micros();

            let mut output_path = input_path.clone();
            output_path.set_extension("out");
            let output = read_as_string(&output_path)[1..].to_vec();

            assert_eq!(solution, output);
            println!("OK, {} sec", elapsed as f64 / 1000000.0);
        }
    }
    Ok(())
}

fn read_as_string(path: &Path) -> Vec<String> {
    let reader = BufReader::new(fs::File::open(path).unwrap());
    reader.lines().map(|x| x.unwrap()).collect()
}
