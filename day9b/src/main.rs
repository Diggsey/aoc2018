use std::fs::File;
use std::io::{BufReader, BufRead};

#[macro_use]
extern crate text_io;

#[derive(Debug, Copy, Clone, Default)]
struct Marble {
    next: usize,
    prev: usize,
}

fn main() {
    let line: String = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let (num_players, mut num_marbles): (usize, usize);
    scan!(line.bytes() => "{} players; last marble is worth {} points", num_players, num_marbles);
    num_marbles = num_marbles*100 + 1;

    let mut scores = vec![0; num_players];
    let mut ring = vec![Marble::default(); num_marbles];
    let mut current = 0;

    for (marble, player) in (1..num_marbles).zip((0..num_players).cycle()) {
        if marble % 23 == 0 {
            for _ in 0..7 {
                current = ring[current].prev;
            }
            let Marble { prev, next } = ring[current];
            scores[player] += marble + current;
            
            ring[prev].next = next;
            ring[next].prev = prev;
            current = next;
        } else {
            let lhs = ring[current].next;
            let rhs = ring[lhs].next;
            ring[lhs].next = marble;
            ring[rhs].prev = marble;
            ring[marble].prev = lhs;
            ring[marble].next = rhs;
            current = marble;
        }
    }

    let max_score = scores.iter().max().unwrap();
    println!("{}", max_score);
}
