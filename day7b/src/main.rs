use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[macro_use]
extern crate text_io;

fn time_for_step(a: char) -> i32 {
    a as i32 + 61 - ('A' as i32)
}

#[derive(Debug, Copy, Clone)]
struct Count {
    incoming: i32,
    time_left: i32,
}

fn main() {
    let mut counts = HashMap::<char, Count>::new();
    let mut edges = HashMap::<char, Vec<char>>::new();

    for line in BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
    {
        let (a, b): (char, char);
        scan!(line.bytes() => "Step {} must be finished before step {} can begin.", a, b);

        counts.entry(a).or_insert_with(|| Count { incoming: 0, time_left: time_for_step(a) });
        counts.entry(b).or_insert_with(|| Count { incoming: 0, time_left: time_for_step(b) }).incoming += 1;
        edges.entry(a).or_default().push(b);
        edges.entry(b).or_insert(Vec::new());
    }

    let mut steps = 0;

    while !counts.is_empty() {
        let mut nodes: Vec<_> = counts.iter().filter(|(_, &v)| v.incoming == 0).map(|(&k, _)| k).collect();
        nodes.sort();

        for n in nodes.into_iter().take(5) {
            let count = counts.get_mut(&n).unwrap();
            count.time_left -= 1;
            if count.time_left == 0 {
                print!("{}", n);
                for m in &edges[&n] {
                    counts.get_mut(m).unwrap().incoming -= 1;
                }
                counts.remove(&n);
            }
        }

        steps += 1;
    }

    println!(" {}", steps);
}
