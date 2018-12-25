use std::fs::File;
use std::io::{BufReader, BufRead};

#[macro_use]
extern crate text_io;

fn main() {
    let line: String = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let (num_players, num_marbles): (usize, usize);
    scan!(line.bytes() => "{} players; last marble is worth {} points", num_players, num_marbles);

    let mut scores = vec![0; num_players];
    let mut ring = vec![0];
    let mut current = 0;

    for (marble, player) in (1..=num_marbles).zip((0..num_players).cycle()) {
        if marble % 23 == 0 {
            current = (current + ring.len() - 7) % ring.len();
            scores[player] += marble + ring.remove(current);
        } else {
            current = (current + 2) % ring.len();
            ring.insert(current, marble);
        }
    }

    let max_score = scores.iter().max().unwrap();
    println!("{}", max_score);
}
