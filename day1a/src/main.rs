use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let result: i32 = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|l| l.parse::<i32>().unwrap())
        .sum();
    println!("{}", result);
}
