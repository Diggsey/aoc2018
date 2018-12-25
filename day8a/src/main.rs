use std::fs::File;
use std::io::{BufReader, BufRead};

fn sum_metadata(slice: &mut &[i32]) -> i32 {
    let num_children = slice[0];
    let num_metadata = slice[1] as usize;
    *slice = &slice[2..];
    let result = (0..num_children).map(|_| sum_metadata(slice)).sum::<i32>() + (&slice[0..num_metadata]).iter().sum::<i32>();
    *slice = &slice[num_metadata..];
    result
}

fn main() {
    let numbers: Vec<i32> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut slice = &numbers[..];
    println!("{}", sum_metadata(&mut slice));

    assert!(slice.is_empty());
}
