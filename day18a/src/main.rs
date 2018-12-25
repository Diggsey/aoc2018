use std::fs::File;
use std::io::{BufReader, BufRead};
use std::mem;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Open,
    Tree,
    Lumber,
}

fn step_grid(grid1: &mut Vec<Vec<Cell>>, grid2: &mut Vec<Vec<Cell>>) {
    for y in 0..grid1.len() {
        for x in 0..grid1[y].len() {
            let neighbours = [
                grid1.get(y.wrapping_sub(1)).and_then(|row| row.get(x.wrapping_sub(1))).cloned().unwrap_or(Cell::Open),
                grid1.get(y.wrapping_sub(1)).and_then(|row| row.get(x)).cloned().unwrap_or(Cell::Open),
                grid1.get(y.wrapping_sub(1)).and_then(|row| row.get(x+1)).cloned().unwrap_or(Cell::Open),
                grid1.get(y).and_then(|row| row.get(x.wrapping_sub(1))).cloned().unwrap_or(Cell::Open),
                grid1.get(y).and_then(|row| row.get(x+1)).cloned().unwrap_or(Cell::Open),
                grid1.get(y+1).and_then(|row| row.get(x.wrapping_sub(1))).cloned().unwrap_or(Cell::Open),
                grid1.get(y+1).and_then(|row| row.get(x)).cloned().unwrap_or(Cell::Open),
                grid1.get(y+1).and_then(|row| row.get(x+1)).cloned().unwrap_or(Cell::Open),
            ];
            let num_trees = neighbours.iter().filter(|&&cell| cell == Cell::Tree).count();
            let num_lumber = neighbours.iter().filter(|&&cell| cell == Cell::Lumber).count();

            grid2[y][x] = match grid1[y][x] {
                Cell::Open if num_trees >= 3 => Cell::Tree,
                Cell::Tree if num_lumber >= 3 => Cell::Lumber,
                Cell::Lumber if num_lumber < 1 || num_trees < 1 => Cell::Open,
                other => other,
            }
        }
    }
    mem::swap(grid1, grid2);
}

fn main() {
    let mut grid1: Vec<Vec<Cell>> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars().map(|c| match c {
                '.' => Cell::Open,
                '|' => Cell::Tree,
                '#' => Cell::Lumber,
                _ => panic!("Unexpected char"),
            }).collect()
        })
        .collect();
    let mut grid2 = grid1.clone();

    for _ in 0..10 {
        step_grid(&mut grid1, &mut grid2);
    }

    let num_trees = grid1.iter().flat_map(|row| row.iter().cloned()).filter(|&cell| cell == Cell::Tree).count();
    let num_lumber = grid1.iter().flat_map(|row| row.iter().cloned()).filter(|&cell| cell == Cell::Lumber).count();

    println!("{}", num_trees*num_lumber);
}
