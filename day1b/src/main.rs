use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn main() {
    let changes: Vec<i32> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|l| l.parse::<i32>().unwrap())
        .collect();
    
    let mut total = 0;
    let mut seen = HashSet::new();
    for &change in changes.iter().cycle() {
        if !seen.insert(total) {
            println!("{}", total);
            break;
        }
        total += change;
    }
}
