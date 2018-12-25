use std::fs::File;
use std::io::{BufReader, BufRead};

#[macro_use]
extern crate text_io;

#[derive(Copy, Clone)]
struct Rect {
    id: i32,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

fn main() {
    let mut rects = Vec::new();

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
        
        rects.push(Rect {
            id,
            x0: x,
            y0: y,
            x1: x+w,
            y1: y+h,
        });
    }

    'outer: for i in 0..rects.len() {
        let rc0 = rects[i];
        for j in 0..rects.len() {
            let rc1 = rects[j];

            if i == j { continue; }
            if rc0.x0 >= rc1.x1 { continue; }
            if rc0.y0 >= rc1.y1 { continue; }
            if rc0.x1 <= rc1.x0 { continue; }
            if rc0.y1 <= rc1.y0 { continue; }

            continue 'outer;
        }

        println!("{}", rc0.id);
        break;
    }
}
