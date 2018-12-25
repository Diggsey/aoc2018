use std::fs::File;
use std::io::{BufReader, BufRead};

fn is_opposite(a: char, b: char) -> bool {
    a.eq_ignore_ascii_case(&b) && a != b
}

fn main() {
    let line = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap();
    
    let mut chars = Vec::new();
    for c in line.chars() {
        if let Some(&prev_c) = chars.last() {
            if is_opposite(c, prev_c) {
                chars.pop();
                continue;
            }
        }
        chars.push(c);
    }

    println!("{} {}", line.len(), chars.len());
}
