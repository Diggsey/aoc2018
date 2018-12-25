use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap, BTreeSet};

#[derive(Debug, Copy, Clone)]
enum Direction {
    N, E, S, W,
}

impl Direction {
    fn apply(self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::N => (pos.0, pos.1-1),
            Direction::E => (pos.0+1, pos.1),
            Direction::S => (pos.0, pos.1+1),
            Direction::W => (pos.0-1, pos.1),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct State {
    block: Vec<Direction>,
    next: Vec<usize>,
}

#[derive(Debug, Clone, Default)]
struct StateMachine {
    states: Vec<State>,
}

impl StateMachine {
    fn add(&mut self) -> usize {
        self.states.push(State::default());
        self.states.len()-1
    }
}

fn parse_regex<I>(iter: &mut I, sm: &mut StateMachine) -> (Vec<usize>, Vec<usize>)
where
    I: Iterator<Item=char>
{
    let mut result = Vec::new();
    let mut initial = Vec::new();
    let mut current = sm.add();
    initial.push(current);

    while let Some(c) = iter.next() {
        match c {
            '|' => {
                result.push(current);
                current = sm.add();
                initial.push(current);
            },
            '(' => {
                let (nested_initial, nested_result) = parse_regex(iter, sm);
                let new_current = sm.add();
                for &nested in &nested_result {
                    sm.states[nested].next = vec![new_current];
                }
                sm.states[current].next = nested_initial;
                current = new_current;
            },
            ')' => {
                break;
            },
            'N' => { sm.states[current].block.push(Direction::N); },
            'E' => { sm.states[current].block.push(Direction::E); },
            'S' => { sm.states[current].block.push(Direction::S); },
            'W' => { sm.states[current].block.push(Direction::W); },
            '^' | '$' => {},
            _ => panic!("Unexpected: {}", c)
        }
    }

    result.push(current);
    (initial, result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cell {
    x: i32,
    y: i32,
    state: usize,
}

type DoorMap = HashMap<(i32, i32), Vec<(i32, i32)>>;

fn add_door(doors: &mut DoorMap, a: (i32, i32), b: (i32, i32)) {
    fn add_half_door(doors: &mut DoorMap, a: (i32, i32), b: (i32, i32)) {
        let v = doors.entry(a).or_default();
        if !v.contains(&b) {
            v.push(b);
        }
    }

    add_half_door(doors, a, b);
    add_half_door(doors, b, a);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct WalkPos {
    dist: usize,
    x: i32,
    y: i32,
}

fn walk(doors: &DoorMap) -> usize {
    let mut visited = HashMap::new();
    let mut pending = BTreeSet::new();

    pending.insert(WalkPos {
        dist: 0, x: 0, y: 0
    });

    while let Some(walk_pos) = pending.iter().cloned().next() {
        pending.remove(&walk_pos);
        if visited.contains_key(&(walk_pos.x, walk_pos.y)) { continue; }
        visited.insert((walk_pos.x, walk_pos.y), walk_pos.dist);

        for &door in doors.get(&(walk_pos.x, walk_pos.y)).unwrap() {
            pending.insert(WalkPos {
                dist: walk_pos.dist + 1,
                x: door.0,
                y: door.1,
            });
        }
    }

    visited.values().cloned().max().unwrap()
}

fn display_map(doors: &DoorMap) {
    let min_x = doors.keys().map(|door| door.0).min().unwrap();
    let min_y = doors.keys().map(|door| door.1).min().unwrap();
    let max_x = doors.keys().map(|door| door.0).max().unwrap();
    let max_y = doors.keys().map(|door| door.1).max().unwrap();
    for y in min_y..=max_y {
        let mut row1 = String::new();
        let mut row2 = String::new();
        for x in min_x..=max_x {
            if let Some(adj) = doors.get(&(x, y)) {
                row1 += ".";
                if adj.iter().any(|door| door.0 > x) {
                    row1 += "|";
                } else {
                    row1 += "#";
                }
                if adj.iter().any(|door| door.1 > y) {
                    row2 += "-";
                } else {
                    row2 += "#";
                }
                row2 += "#";
            } else {
                row1 += "##";
                row2 += "##";
            }
        }
        println!("{}", row1);
        println!("{}", row2);
    }
}

fn main() {
    let regex = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap();
    
    let mut sm = StateMachine::default();
    parse_regex(regex.chars().by_ref(), &mut sm);

    let mut visited_cells = HashSet::new();
    let mut pending_cells = HashSet::new();
    let mut doors = HashMap::new();

    pending_cells.insert(Cell {
        x: 0, y: 0, state: 0,
    });
    visited_cells.insert(Cell {
        x: 0, y: 0, state: 0,
    });

    while let Some(cell) = pending_cells.iter().cloned().next() {
        pending_cells.remove(&cell);
        let mut pos = (cell.x, cell.y);
        let state = &sm.states[cell.state];
        for &dir in &state.block {
            let new_pos = dir.apply(pos);
            add_door(&mut doors, pos, new_pos);
            pos = new_pos;
        }
        for &next in &state.next {
            let new_cell = Cell {
                x: pos.0, y: pos.1, state: next,
            };
            if !visited_cells.contains(&new_cell) {
                pending_cells.insert(new_cell);
                visited_cells.insert(new_cell);
            }
        }
    }

    display_map(&doors);
    let result = walk(&doors);
    println!("{:?}", sm);
    println!("{}", result);
}
