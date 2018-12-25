use std::fs::File;
use std::io::{BufReader, BufRead};

#[macro_use]
extern crate text_io;

struct Nanobot {
    pos: (i32, i32, i32),
    radius: i32,
}

fn test_pos(pos: (i32, i32, i32), nanobots: &[Nanobot]) -> usize {
    nanobots.iter().filter(|nanobot| {
        let (dx, dy, dz) = (nanobot.pos.0 - pos.0, nanobot.pos.1 - pos.1, nanobot.pos.2 - pos.2);
        let dist = dx.abs() + dy.abs() + dz.abs();
        dist <= nanobot.radius
    }).count()
}

fn adjacent_candidates(pos: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut result = Vec::new();
    if pos.0 != 0 { result.push((pos.0 - pos.0.signum(), pos.1, pos.2)); }
    if pos.1 != 0 { result.push((pos.0, pos.1 - pos.1.signum(), pos.2)); }
    if pos.2 != 0 { result.push((pos.0, pos.1, pos.2 - pos.2.signum())); }
    if pos.0 != 0 && pos.1 != 0 { result.push((pos.0 - pos.0.signum(), pos.1 - pos.1.signum(), pos.2)); }
    if pos.1 != 0 && pos.2 != 0 { result.push((pos.0, pos.1 - pos.1.signum(), pos.2 - pos.2.signum())); }
    if pos.2 != 0 && pos.0 != 0 { result.push((pos.0 - pos.0.signum(), pos.1, pos.2 - pos.2.signum())); }
    result
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
    
    let candidates: Vec<_> = nanobots.iter().flat_map(|nanobot| {
        vec![
            (nanobot.pos.0, nanobot.pos.1, nanobot.pos.2 - nanobot.radius),
            (nanobot.pos.0, nanobot.pos.1, nanobot.pos.2 + nanobot.radius),
            (nanobot.pos.0, nanobot.pos.1 - nanobot.radius, nanobot.pos.2),
            (nanobot.pos.0, nanobot.pos.1 + nanobot.radius, nanobot.pos.2),
            (nanobot.pos.0 - nanobot.radius, nanobot.pos.1, nanobot.pos.2),
            (nanobot.pos.0 + nanobot.radius, nanobot.pos.1, nanobot.pos.2),
        ]
    }).map(|pos| (pos, test_pos(pos, &nanobots))).collect();


    let mut max_n = *candidates.iter().map(|(_pos, n)| n).max().unwrap();
    let best: Vec<_> = candidates.iter().filter(|&&(_pos, n)| n == max_n).collect();

    println!("{:?}", best);
    let mut best_pos = best[0].0;

    println!("{}", best_pos.0.abs() + best_pos.1.abs() + best_pos.2.abs());

    'step: loop {
        for candidate in adjacent_candidates(best_pos) {
            let n = test_pos(candidate, &nanobots);
            if n >= max_n {
                if n > max_n {
                    println!("Better N: {}", n);
                    max_n = n;
                }
                best_pos = candidate;
                continue 'step;
            }
        }
        break;
    }

    println!("{:?}", best_pos);
    println!("{}", best_pos.0.abs() + best_pos.1.abs() + best_pos.2.abs());
}
