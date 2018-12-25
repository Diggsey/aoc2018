use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt::Write;
use std::collections::BTreeSet;
use std::thread;
use std::time::Duration;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum EntityType {
    Elf,
    Goblin,
}

impl EntityType {
    fn enemy(self) -> Self {
        match self {
            EntityType::Elf => EntityType::Goblin,
            EntityType::Goblin => EntityType::Elf,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum CellType {
    Empty,
    Wall,
    Entity {
        type_: EntityType,
        power: i32,
        health: i32,
        moved: bool,
    }
}

#[derive(Copy, Clone, Debug)]
struct Cell {
    type_: CellType,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum MoveAction {
    Move { dist: i32, dest_y: usize, dest_x: usize, y: usize, x: usize },
    None,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum AttackAction {
    Attack { health: i32, y: usize, x: usize },
    None,
}

const SAVE_CURSOR: &str = "\x1B[s";
const RESTORE_CURSOR: &str = "\x1B[u";
const CLEAR_SCREEN: &str = "\x1B[J";
const RESET_COLORS: &str = "\x1B[m";
const SET_FG_WHITE: &str = "\x1B[90m";
const SET_FG_RED: &[&str] = &["\x1B[91m", "\x1B[31m"];
const SET_FG_GREEN: &[&str] = &["\x1B[92m", "\x1B[32m"];
const HEALTH_BARS: &[char] = &[' ', '░', '▒', '▓', '█'];

impl Cell {
    fn new(c: char) -> Self {
        Cell {
            type_: match c {
                '.' => CellType::Empty,
                '#' => CellType::Wall,
                'G' => CellType::Entity {
                    type_: EntityType::Goblin,
                    power: 3,
                    health: 200,
                    moved: false,
                },
                'E' => CellType::Entity {
                    type_: EntityType::Elf,
                    power: 12,
                    health: 200,
                    moved: false,
                },
                _ => panic!("Unknown character: {}", c),
            }
        }
    }
    fn display(&self, result: &mut String, info: &mut String) {
        match self.type_ {
            CellType::Empty => { write!(result, "  "); }
            CellType::Wall => { write!(result, "██"); }
            CellType::Entity { type_, health, moved, .. } => {
                let (color, letter) = match type_ {
                    EntityType::Elf => (SET_FG_GREEN[moved as usize], "E"),
                    EntityType::Goblin => (SET_FG_RED[moved as usize], "G"),
                };
                let bar = HEALTH_BARS[((health-1)/40) as usize];
                write!(result, "{}{}{}{}", color, letter, bar, SET_FG_WHITE);
                if !info.is_empty() {
                    write!(info, ", ");
                }
                write!(info, "{}({})", letter, health);
            }
        }
    }
}

fn find_move(grid: &[Vec<Cell>], pos: (usize, usize), enemy_type: EntityType) -> MoveAction {
    let mut queue = BTreeSet::new();
    let mut visited = BTreeSet::new();
    let mut solutions: Vec<(i32, (usize, usize))> = Vec::new();
    queue.insert((1, pos));

    while let Some(&min_key) = queue.iter().next() {
        queue.remove(&min_key);
        let (dist, cur_pos) = min_key;

        if !solutions.is_empty() {
            if dist > solutions[0].0 {
                break;
            }
        }

        visited.insert(cur_pos);
        match grid[cur_pos.0][cur_pos.1].type_ {
            CellType::Empty => {
                let adjacents = &[
                    (cur_pos.0-1, cur_pos.1),
                    (cur_pos.0, cur_pos.1-1),
                    (cur_pos.0+1, cur_pos.1),
                    (cur_pos.0, cur_pos.1+1),
                ];
                for &adjacent in adjacents {
                    if visited.contains(&adjacent) { continue; }
                    queue.insert((dist + 1, adjacent));
                }
            },
            CellType::Entity { type_, .. } => if type_ == enemy_type {
                solutions.push((dist, cur_pos))
            },
            _ => {}
        }

    }

    if solutions.is_empty() {
        MoveAction::None
    } else {
        solutions.sort();
        MoveAction::Move {
            dist: solutions[0].0,
            dest_y: (solutions[0].1).0,
            dest_x: (solutions[0].1).1,
            y: pos.0,
            x: pos.1,
        }
    }
}

fn find_attack(grid: &[Vec<Cell>], pos: (usize, usize), enemy_type: EntityType) -> AttackAction {
    match grid[pos.0][pos.1].type_ {
        CellType::Entity { type_, health, .. } => if type_ == enemy_type {
            AttackAction::Attack { health, y: pos.0, x: pos.1 }
        } else {
            AttackAction::None
        },
        _ => AttackAction::None
    }
}

fn display_grid(grid: &[Vec<Cell>], round: i32) {
    let mut result = String::new();
    writeln!(&mut result, "{}{}{}", RESTORE_CURSOR, CLEAR_SCREEN, SET_FG_WHITE);
    for row in grid {
        let mut info = String::new();
        for cell in row {
            cell.display(&mut result, &mut info);
        }
        writeln!(&mut result, " {}", info);
    }
    writeln!(&mut result, "{}", RESET_COLORS);

    // Calculate remaining hitpoints
    let mut total_health = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if let CellType::Entity { health, .. } = grid[y][x].type_ {
                total_health += health;
            }
        }
    }
    writeln!(&mut result, "{}: {} - {}", round, total_health, round*total_health);

    println!("{}", result);
}

fn exists_target(grid: &[Vec<Cell>], enemy_type: EntityType) -> bool {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if let CellType::Entity { type_, .. } = grid[y][x].type_ {
                if type_ == enemy_type {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    let arg = std::env::args().skip(1).next().unwrap_or_default();
    let mut grid: Vec<Vec<Cell>> = BufReader::new(File::open(&format!("input{}.txt", arg)).unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars()
            .map(Cell::new)
            .collect()
        )
        .collect();

    println!();
    print!("{}", SAVE_CURSOR);

    let speed = Duration::from_millis(200);

    display_grid(&grid, 0);
    thread::sleep(speed);

    for round in 0.. {
        let mut finished = false;
        let mut failed = false;
        // Perform movements
        'done: for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if let CellType::Entity { type_, ref mut moved, power, .. } = grid[y][x].type_ {
                    if *moved { continue; }
                    *moved = true;
                    let enemy_type = type_.enemy();

                    if !exists_target(&grid, enemy_type) {
                        finished = true;
                        break 'done;
                    }

                    let move_action = *[
                        find_move(&grid, (y-1, x), enemy_type),
                        find_move(&grid, (y, x-1), enemy_type),
                        find_move(&grid, (y+1, x), enemy_type),
                        find_move(&grid, (y, x+1), enemy_type),
                    ].iter().min().unwrap();

                    match move_action {
                        MoveAction::Move { y: mut y2, x: mut x2, dist, .. } => {
                            if dist > 1 {
                                grid[y2][x2].type_ = grid[y][x].type_;
                                grid[y][x].type_ = CellType::Empty;
                            } else {
                                x2 = x;
                                y2 = y;
                            }
                                    
                            let attack_action = *[
                                find_attack(&grid, (y2-1, x2), enemy_type),
                                find_attack(&grid, (y2, x2-1), enemy_type),
                                find_attack(&grid, (y2+1, x2), enemy_type),
                                find_attack(&grid, (y2, x2+1), enemy_type),
                            ].iter().min().unwrap();

                            match attack_action {
                                AttackAction::Attack { y: y3, x: x3, .. } => {
                                    if let CellType::Entity { ref mut health, type_, .. } = grid[y3][x3].type_ {
                                        *health -= power;
                                        if *health <= 0 {
                                            grid[y3][x3].type_ = CellType::Empty;
                                            if type_ == EntityType::Elf {
                                                failed = true;
                                                break 'done;
                                            }
                                        }
                                    } else {
                                        panic!("Expected entity");
                                    }
                                },
                                AttackAction::None => {}
                            }

                        },
                        MoveAction::None => {}
                    }

                    //display_grid(&grid, round);
                    //thread::sleep(speed);
                }
            }
        }
        // Reset moved flags
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if let CellType::Entity { ref mut moved, .. } = grid[y][x].type_ {
                    *moved = false;
                }
            }
        }

        display_grid(&grid, round);
        //thread::sleep(speed);
        let _ = std::io::stdin().read_line(&mut String::new()).unwrap();

        if finished || failed {
            if failed {
                println!("FAILED!");
            }
            break;
        }
    }
}
