use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let mut c2 = 0;
    let mut c3 = 0;

    for line in BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
    {
        let mut m = HashMap::new();
        for c in line.chars() {
            *m.entry(c).or_insert(0) += 1;
        }

        let mut f2 = false;
        let mut f3 = false;
        for &v in m.values() {
            if v == 2 { f2 = true; }
            if v == 3 { f3 = true; }
        }

        if f2 { c2 += 1; }
        if f3 { c3 += 1; }
    }
    
    println!("{}", c2*c3);
}
