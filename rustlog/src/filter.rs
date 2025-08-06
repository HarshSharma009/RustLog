/// Filters lines from a vector of strings based on a keyword.
/// Returns a new vector containing only the lines that contain the keyword.
use std::vec::Vec;
pub fn filter_lines(lines: Vec<String>, keyword: &str) -> Vec<String> {
    lines
        .into_iter()
        .filter(|line| line.contains(keyword))
        .collect()
}