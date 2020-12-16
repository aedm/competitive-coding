use crate::utils::read_lines;

pub fn solve() -> String {
    let lines = read_lines("scratch.txt");
    lines.join("_")
}
