use std::fs::File;
use std::io::{BufReader, BufRead};

fn node_value(slice: &mut &[i32]) -> i32 {
    let num_children = slice[0];
    let num_metadata = slice[1] as usize;
    *slice = &slice[2..];
    let child_values: Vec<i32> = (0..num_children).map(|_| node_value(slice)).collect();
    let result = if child_values.is_empty() {
        slice[0..num_metadata].iter().sum::<i32>()
    } else {
        slice[0..num_metadata].iter().filter_map(|&m| {
            child_values.get((m-1) as usize)
        }).sum::<i32>()
    };
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
    println!("{}", node_value(&mut slice));

    assert!(slice.is_empty());
}
