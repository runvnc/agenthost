use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn cat_files(file1: &str, file2: &str) -> io::Result<String> {
    let mut result = String::new();

    let file1 = File::open(file1)?;
    let reader1 = BufReader::new(file1);
    for line in reader1.lines() {
        result.push_str(&line?);
        result.push('\n');
    }

    let file2 = File::open(file2)?;
    let reader2 = BufReader::new(file2);
    for line in reader2.lines() {
        result.push_str(&line?);
        result.push('\n');
    }

    Ok(result)
}
