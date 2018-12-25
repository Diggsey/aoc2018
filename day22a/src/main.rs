use std::fs::File;
use std::io::{BufReader, BufRead};

#[macro_use]
extern crate text_io;

fn main() {
    let mut line_iter = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap);
    
    let (depth, target_x, target_y): (usize, usize, usize);
    let depth_line = line_iter.next().unwrap();
    scan!(depth_line.bytes() => "depth: {}", depth);
    let target_line = line_iter.next().unwrap();
    scan!(target_line.bytes() => "target: {},{}", target_x, target_y);

    let mut erosion_levels = vec![vec![0usize; target_x+1]; target_y+1];

    for y in 0..=target_y {
        for x in 0..=target_x {
            let geo_index = if x == 0 || y == 0 {
                x*16807 + y*48271
            } else if x == target_x && y == target_y {
                0
            } else {
                erosion_levels[y][x-1] * erosion_levels[y-1][x]
            };
            erosion_levels[y][x] = (geo_index + depth) % 20183;
        }
    }

    let risk_level: usize = erosion_levels
        .iter()
        .flat_map(|row| row.iter())
        .cloned()
        .map(|erosion_level| erosion_level % 3)
        .sum();
    println!("{}", risk_level);
}
