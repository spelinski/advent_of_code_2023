use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn vector_of_lines(filepath: &Path) -> Vec<String> {
    let file = File::open(filepath).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}