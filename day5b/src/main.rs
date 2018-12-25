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
    
    let result = (b'a'..b'z').map(|rem_byte| {
        let mut chars = Vec::new();
        for c in line.chars().filter(|x| !x.eq_ignore_ascii_case(&(rem_byte as char))) {
            if let Some(&prev_c) = chars.last() {
                if is_opposite(c, prev_c) {
                    chars.pop();
                    continue;
                }
            }
            chars.push(c);
        }
        chars.len()
    }).min().unwrap();

    println!("{}", result);
}
