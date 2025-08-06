use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;

pub fn read_lines<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}