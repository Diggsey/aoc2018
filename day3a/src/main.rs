use std::fs::File;
use std::io::{BufReader, BufRead};

#[macro_use]
extern crate text_io;

struct Edge {
    x: i32,
    from: i32,
    to: i32,
    inc: i32,
}

fn main() {
    let mut edges = Vec::new();

    for line in BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
    {
        let id: i32;
        let x: i32;
        let y: i32;
        let w: i32;
        let h: i32;
        scan!(line.bytes() => "#{} @ {},{}: {}x{}", id, x, y, w, h);
        
        edges.push(Edge {
            x,
            from: y,
            to: y+h,
            inc: 1,
        });
        edges.push(Edge {
            x: x+w,
            from: y,
            to: y+h,
            inc: -1,
        });
    }

    edges.sort_by_key(|e| e.x);

    let mut x = 0;
    let mut cells = vec![0i32; 1000];
    let mut area = 0;
    for edge in edges {
        if edge.x > x {
            let w = edge.x - x;
            let h = cells.iter().filter(|&&c| c >= 2).count() as i32;
            area += w*h;
            x = edge.x;
        }
        for y in edge.from..edge.to {
            cells[y as usize] += edge.inc;
        }
    }

    println!("{}", area);
}
