use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn main() {
    let mut h = HashSet::new();

    for line in BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
    {
        for (index, _) in line.char_indices() {
            let mut v = line.clone();
            v.remove(index);
            v.insert(index, '_');
            if !h.insert(v.clone()) {
                println!("{}", v);
                return;
            }
        }
    }
}
