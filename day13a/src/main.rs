use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Copy, Clone)]
enum CellType {
    Empty,
    Vertical,
    Horizontal,
    ForwardSlash,
    BackSlash,
    Cross,
}

impl CellType {
    fn apply(self, dir: Direction, state: &mut u32) -> Direction {
        match self {
            CellType::Vertical | CellType::Horizontal => dir,
            CellType::ForwardSlash => match dir {
                Direction::North => Direction::East,
                Direction::East => Direction::North,
                Direction::South => Direction::West,
                Direction::West => Direction::South,
            },
            CellType::BackSlash => match dir {
                Direction::North => Direction::West,
                Direction::West => Direction::North,
                Direction::South => Direction::East,
                Direction::East => Direction::South,
            },
            CellType::Cross => {
                *state = (*state + 1) % 3;
                match (dir, state) {
                    (dir, 2) => dir,
                    (Direction::North, 0) => Direction::East,
                    (Direction::North, 1) => Direction::West,
                    (Direction::East, 0) => Direction::South,
                    (Direction::East, 1) => Direction::North,
                    (Direction::South, 0) => Direction::West,
                    (Direction::South, 1) => Direction::East,
                    (Direction::West, 0) => Direction::North,
                    (Direction::West, 1) => Direction::South,
                    _ => unreachable!()
                }
            },
            CellType::Empty => panic!("Bad grid!")
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    cell_type: CellType,
    occupied: bool,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn apply(self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (pos.0 - 1, pos.1),
            Direction::East => (pos.0, pos.1 + 1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::West => (pos.0, pos.1 - 1),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Cart {
    pos: (usize, usize),
    dir: Direction,
    state: u32,
}

fn main() {
    let mut carts = Vec::new();

    let mut grid: Vec<_> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(|(x, c)| {
                let (cell_type, maybe_dir) = match c {
                    '-' => (CellType::Horizontal, None),
                    '|' => (CellType::Vertical, None),
                    '\\' => (CellType::BackSlash, None),
                    '/' => (CellType::ForwardSlash, None),
                    '+' => (CellType::Cross, None),
                    '>' => (CellType::Horizontal, Some(Direction::East)),
                    '<' => (CellType::Horizontal, Some(Direction::West)),
                    '^' => (CellType::Vertical, Some(Direction::North)),
                    'v' => (CellType::Vertical, Some(Direction::South)),
                    ' ' => (CellType::Empty, None),
                    _ => panic!("Unexpected character: {}", c)
                };
                let mut cell = Cell { cell_type, occupied: false };
                if let Some(dir) = maybe_dir {
                    cell.occupied = true;
                    carts.push(Cart {
                        pos: (y, x),
                        dir,
                        state: 0,
                    })
                }
                cell
            }).collect::<Vec<_>>()
        })
        .collect();
    
    loop {
        carts.sort_by_key(|cart| cart.pos);
        for cart in &mut carts {
            grid[cart.pos.0][cart.pos.1].occupied = false;
            cart.pos = cart.dir.apply(cart.pos);
            if grid[cart.pos.0][cart.pos.1].occupied {
                println!("Crash: {}, {}", cart.pos.1, cart.pos.0);
                return;
            }
            grid[cart.pos.0][cart.pos.1].occupied = true;
            cart.dir = grid[cart.pos.0][cart.pos.1].cell_type.apply(cart.dir, &mut cart.state);
        }
    }
}
