use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[macro_use]
extern crate text_io;

fn main() {
    let mut counts = HashMap::<char, i32>::new();
    let mut edges = HashMap::<char, Vec<char>>::new();

    for line in BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
    {
        let (a, b): (char, char);
        scan!(line.bytes() => "Step {} must be finished before step {} can begin.", a, b);

        counts.entry(a).or_insert(0);
        *counts.entry(b).or_default() += 1;
        edges.entry(a).or_default().push(b);
        edges.entry(b).or_insert(Vec::new());
    }

    while let Some(&n) = counts.iter().filter(|(_, &v)| v == 0).map(|(k, _)| k).min() {
        print!("{}", n);
        for m in &edges[&n] {
            *counts.get_mut(m).unwrap() -= 1;
        }
        counts.remove(&n);
    }

    assert!(counts.is_empty());
}
