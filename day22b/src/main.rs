use std::fs::File;
use std::io::{BufReader, BufRead};

use std::collections::{BTreeSet, HashSet};

#[macro_use]
extern crate text_io;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Tool {
    Neither,
    Torch,
    ClimbingGear,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum CaveType {
    Rocky,
    Wet,
    Narrow,
}

impl From<usize> for CaveType {
    fn from(other: usize) -> Self {
        match other % 3 {
            0 => CaveType::Rocky,
            1 => CaveType::Wet,
            2 => CaveType::Narrow,
            _ => unreachable!()
        }
    }
}

impl Tool {
    fn usable_in(self, cave_type: CaveType) -> bool {
        match self {
            Tool::Neither => cave_type != CaveType::Rocky,
            Tool::Torch => cave_type != CaveType::Wet,
            Tool::ClimbingGear => cave_type != CaveType::Narrow,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct PendingCell {
    time: usize,
    x: usize,
    y: usize,
    tool: Tool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct VisitedCell {
    x: usize,
    y: usize,
    tool: Tool,
}

fn main() {
    let mut line_iter = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap);
    
    let (depth, target_x, target_y): (usize, usize, usize);
    let depth_line = line_iter.next().unwrap();
    scan!(depth_line.bytes() => "depth: {}", depth);
    let target_line = line_iter.next().unwrap();
    scan!(target_line.bytes() => "target: {},{}", target_x, target_y);

    let max_x = 1000;
    let max_y = 1000;
    let mut erosion_levels = vec![vec![0usize; max_x]; max_y];

    for y in 0..max_y {
        for x in 0..max_x {
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

    let caves: Vec<Vec<CaveType>> = erosion_levels.into_iter()
        .map(|row| row.into_iter().map(Into::into).collect())
        .collect();
    
    let mut pending_cells = BTreeSet::new();
    let mut visited_cells = HashSet::new();

    pending_cells.insert(PendingCell {
        time: 0,
        x: 0,
        y: 0,
        tool: Tool::Torch,
    });

    while let Some(cell) = pending_cells.iter().next().cloned() {
        if cell.x == target_x && cell.y == target_y {
            println!("{}", cell.time);
            break;
        }
        pending_cells.remove(&cell);
        if !visited_cells.insert(VisitedCell {
            x: cell.x,
            y: cell.y,
            tool: cell.tool,
        }) { continue; }

        // We can switch tools
        for &new_tool in &[Tool::Neither, Tool::Torch, Tool::ClimbingGear] {
            if new_tool.usable_in(caves[cell.y][cell.x]) && new_tool != cell.tool {
                pending_cells.insert(PendingCell {
                    time: cell.time + 7,
                    x: cell.x,
                    y: cell.y,
                    tool: new_tool,
                });
            }
        }

        // Or we can move
        for &movement in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if cell.x == 0 && movement.0 < 0 { continue; }
            if cell.y == 0 && movement.1 < 0 { continue; }

            let (new_x, new_y) = (
                (cell.x as isize + movement.0) as usize,
                (cell.y as isize + movement.1) as usize,
            );
            if cell.tool.usable_in(caves[new_y][new_x]) {
                pending_cells.insert(PendingCell {
                    time: cell.time + 1,
                    x: new_x,
                    y: new_y,
                    tool: cell.tool,
                });
            }
        }
    }
}
