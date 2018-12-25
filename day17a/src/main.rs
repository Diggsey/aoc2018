use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

#[macro_use]
extern crate text_io;

#[derive(Debug, Copy, Clone)]
enum Line {
    Row {
        y: usize,
        x0: usize,
        x1: usize,
    },
    Col {
        x: usize,
        y0: usize,
        y1: usize,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Solid,
    FlowingWater,
    StillWater,
}

fn scan_row(line: &str) -> Result<Line, Box<Error>> {
    let (y, x0, x1): (usize, usize, usize);
    try_scan!(line.bytes() => "y={}, x={}..{}", y, x0, x1);
    Ok(Line::Row { y, x0, x1 })
}

fn scan_col(line: &str) -> Result<Line, Box<Error>> {
    let (x, y0, y1): (usize, usize, usize);
    try_scan!(line.bytes() => "x={}, y={}..{}", x, y0, y1);
    Ok(Line::Col { x, y0, y1 })
}

fn fill(grid: &mut [Vec<Cell>], x: usize, y: usize) -> bool {
    if y >= grid.len()-1 { return false; }

    grid[y][x] = Cell::FlowingWater;
    if grid[y+1][x] == Cell::FlowingWater || (grid[y+1][x] == Cell::Empty && !fill(grid, x, y+1)) {
        false
    } else {
        let mut fill_right = None;
        for dx in 1.. {
            match grid[y][x+dx] {
                Cell::Solid => { fill_right = Some(dx-1); break; },
                Cell::Empty => {},
                Cell::FlowingWater => { break; },
                Cell::StillWater => {
                    display(grid);
                    panic!();
                },
            }

            grid[y][x+dx] = Cell::FlowingWater;
            match grid[y+1][x+dx] {
                Cell::Solid | Cell::StillWater => {},
                Cell::Empty => { if !fill(grid, x+dx, y+1) { break; } },
                Cell::FlowingWater => { break; }
            }
        }

        let mut fill_left = None;
        for dx in 1.. {
            match grid[y][x-dx] {
                Cell::Solid => { fill_left = Some(dx-1); break; },
                Cell::Empty => {},
                Cell::FlowingWater => { break; },
                Cell::StillWater => {
                    display(grid);
                    panic!();
                },
            }

            grid[y][x-dx] = Cell::FlowingWater;
            match grid[y+1][x-dx] {
                Cell::Solid | Cell::StillWater => {},
                Cell::Empty => { if !fill(grid, x-dx, y+1) { break; } },
                Cell::FlowingWater => { break; }
            }
        }

        if let (Some(lx), Some(rx)) = (fill_left, fill_right) {
            for x2 in (x-lx)..=(x+rx) {
                grid[y][x2] = Cell::StillWater;
            }
            true
        } else {
            false
        }
    }
}

fn display(grid: &[Vec<Cell>]) {
    for row in grid {
        let s: String = row.iter().map(|c| match c {
            Cell::Empty => '.',
            Cell::Solid => '#',
            Cell::FlowingWater => '|',
            Cell::StillWater => '~',
        }).collect();
        println!("{}", s);
    }
}

fn main2() {
    let lines: Vec<_> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            scan_row(&line).or_else(|_| scan_col(&line)).unwrap()
        })
        .collect();
    let min_x = lines.iter().map(|line| match line {
        Line::Row { x0, .. } => x0,
        Line::Col { x, .. } => x,
    }).min().unwrap() - 2;
    let max_x = lines.iter().map(|line| match line {
        Line::Row { x1, .. } => x1,
        Line::Col { x, .. } => x,
    }).max().unwrap() + 2;
    let min_y = lines.iter().map(|line| match line {
        Line::Row { y, .. } => y,
        Line::Col { y0, .. } => y0,
    }).min().unwrap() - 1;
    let max_y = lines.iter().map(|line| match line {
        Line::Row { y, .. } => y,
        Line::Col { y1, .. } => y1,
    }).max().unwrap() + 2;

    let w = max_x + 1 - min_x;
    let h = max_y + 2 - min_y;

    let mut grid = vec![vec![Cell::Empty; w]; h];

    for line in lines {
        match line {
            Line::Row { y, x0, x1 } => {
                for x in x0..=x1 {
                    grid[y-min_y][x-min_x] = Cell::Solid;
                }
            },
            Line::Col { x, y0, y1 } => {
                for y in y0..=y1 {
                    grid[y-min_y][x-min_x] = Cell::Solid;
                }
            },
        }
    }

    //display(&grid);
    fill(&mut grid, 500-min_x, 0);
    display(&grid);

    let result = (1..(grid.len()-3)).flat_map(|y| (0..w).map(move |x| (x, y))).filter(|&(x, y)| match grid[y][x] {
        Cell::FlowingWater | Cell::StillWater => true,
        _ => false,
    }).count();

    println!("{}", result);
}

fn main() {
    use std::thread;
    thread::Builder::new()
        .stack_size(1024*1024*512)
        .spawn(|| main2())
        .unwrap()
        .join()
        .unwrap();
}
