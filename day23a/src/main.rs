use std::fs::File;
use std::io::{BufReader, BufRead};

#[macro_use]
extern crate text_io;

struct Nanobot {
    pos: (i32, i32, i32),
    radius: i32,
}

fn main() {
    let nanobots: Vec<_> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let x: i32;
            let y: i32;
            let z: i32;
            let r: i32;
            scan!(line.bytes() => "pos=<{},{},{}>, r={}", x, y, z, r);

            Nanobot {
                pos: (x, y, z),
                radius: r,
            }
        })
        .collect();
    
    let best = nanobots.iter().max_by_key(|nanobot| nanobot.radius).unwrap();
    let pos = best.pos;

    let n = nanobots.iter().filter(|nanobot| {
        let (dx, dy, dz) = (nanobot.pos.0 - pos.0, nanobot.pos.1 - pos.1, nanobot.pos.2 - pos.2);
        let dist = dx.abs() + dy.abs() + dz.abs();
        dist <= best.radius
    }).count();

    println!("{}", n);
}
