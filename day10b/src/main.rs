use std::fs::File;
use std::io::{BufReader, BufRead};

#[macro_use]
extern crate text_io;

fn main() {
    let mut result: Vec<_> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let l = line.replace(" ", "");
            let (px, py, vx, vy): (i32, i32, i32, i32);
            scan!(l.bytes() => "position=<{},{}>velocity=<{},{}>", px, py, vx, vy);
            (px, py, vx, vy)
        })
        .collect();
    
    for i in 1..20000 {
        let mut avgx: i32 = result.iter().map(|(px, _, _, _)| px).sum();
        let mut avgy: i32 = result.iter().map(|(_, py, _, _)| py).sum();
        avgx /= result.len() as i32;
        avgy /= result.len() as i32;

        let mut grid = [[false; 100]; 100];
        let mut found = false;
        for item in &mut result {
            item.0 += item.2;
            item.1 += item.3;

            let (x, y) = ((item.0 + 50 - avgx) as usize, (item.1 + 50 - avgy) as usize);
            if x < grid[0].len() && y < grid.len() {
                grid[y][x] = true;
                found = true;
            }
        }

        if found {
            println!("{}", i);
            for y in 0..grid.len() {
                let mut line = String::new();
                for x in 0..grid[y].len() {
                    if grid[y][x] {
                        line += "X";
                    } else {
                        line += " ";
                    }
                }
                println!("{}", line);
            }
        }
    }
}
