use std::fs::File;
use std::io::{BufReader, BufRead};

#[macro_use]
extern crate text_io;

#[derive(Debug, Copy, Clone)]
struct Cell {
    index: i32,
    dist: i32,
}

fn main() {
    let coords: Vec<_> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let x: i32;
            let y: i32;
            scan!(line.bytes() => "{}, {}", x, y);
            (x, y)
        })
        .collect();
    
    let min_x = coords.iter().map(|(x, _)| x).min().unwrap();
    let min_y = coords.iter().map(|(_, y)| y).min().unwrap();
    let max_x = coords.iter().map(|(x, _)| x).max().unwrap();
    let max_y = coords.iter().map(|(_, y)| y).max().unwrap();

    let margin_x = (max_x - min_x)+5;
    let margin_y = (max_y - min_y)+5;

    let min_bounds = (min_x - margin_x, min_y - margin_y);
    let max_bounds = (max_x + margin_x, max_y + margin_y);
    let offset = (-min_bounds.0, -min_bounds.1);
    let size = (max_bounds.0 - min_bounds.0, max_bounds.1 - min_bounds.1);

    // Find the cell closest to each coordinate
    let mut values = vec![Cell { index: -1, dist: ::std::i32::MAX }; (size.0 * size.1) as usize];
    for (ci, (mut cx, mut cy)) in coords.iter().enumerate() {
        cx += offset.0;
        cy += offset.1;
        for x in 0..size.0 {
            for y in 0..size.1 {
                let d = (x - cx).abs() + (y - cy).abs();
                let cell = &mut values[(y*size.0+x) as usize];
                if d < cell.dist {
                    cell.dist = d;
                    cell.index = ci as i32;
                } else if d == cell.dist {
                    cell.index = -2;
                }
            }
        }
    }

    // Count the areas of each coordinate
    let mut counts = vec![0i32; coords.len()];
    for cell in values.iter() {
        if cell.index >= 0 {
            counts[cell.index as usize] += 1;
        }
    }

    // Exclude coordinates whose areas hit the boundary
    for x in 0..size.0 {
        let cell = values[x as usize];
        if cell.index >= 0 {
            counts[cell.index as usize] = 0;
        }
        let cell = values[((size.1 - 1)*size.0 + x) as usize];
        if cell.index >= 0 {
            counts[cell.index as usize] = 0;
        }
    }

    for y in 0..size.1 {
        let cell = values[(y*size.0) as usize];
        if cell.index >= 0 {
            counts[cell.index as usize] = 0;
        }
        let cell = values[(y*size.0 + size.1 - 1) as usize];
        if cell.index >= 0 {
            counts[cell.index as usize] = 0;
        }
    }

    // Find the largest area
    let max_count = counts.iter().max().unwrap();
    println!("{}", max_count);
}
