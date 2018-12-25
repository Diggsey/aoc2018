use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[macro_use]
extern crate text_io;

fn resolve(index: &mut usize, map: &mut HashMap<usize, usize>) {
    if let Some(&(mut parent)) = map.get(index) {
        resolve(&mut parent, map);
        map.insert(*index, parent);
        *index = parent;
    }
}

fn merge(mut a: usize, mut b: usize, map: &mut HashMap<usize, usize>) {
    resolve(&mut a, map);
    resolve(&mut b, map);
    if a == b { return }
    map.insert(a, b);
}

const OFFSET: i32 = 8;
fn index_grid(grid: &mut [[[[Option<usize>; 17]; 17]; 17]], x: i32, y: i32, z: i32, w: i32) -> &mut Option<usize> {
    &mut grid[(x + OFFSET) as usize][(y + OFFSET) as usize][(z + OFFSET) as usize][(w + OFFSET) as usize]
}

fn in_range(coord: i32) -> bool {
    coord >= -OFFSET && coord <= OFFSET
}

fn main() {
    let mut grid = vec![[[[None; 17]; 17]; 17]; 17];
    let mut map = HashMap::new();
    let mut num_stars = 0;
    for (index, line) in BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .enumerate()
    {
        let (x, y, z, w): (i32, i32, i32, i32);
        scan!(line.bytes() => "{},{},{},{}", x, y, z, w);

        let cell = index_grid(&mut grid, x, y, z, w);
        assert!(cell.is_none());
        *cell = Some(index);
        num_stars += 1;

        let mx = 3i32;
        for dx in -mx..=mx {
            if !in_range(x + dx) { continue; }
            let my = mx-dx.abs();
            for dy in -my..=my {
                if !in_range(y + dy) { continue; }
                let mz = my-dy.abs();
                for dz in -mz..=mz {
                    if !in_range(z + dz) { continue; }
                    let mw = mz-dz.abs();
                    for dw in -mw..=mw {
                        if !in_range(w + dw) { continue; }
                        if let Some(other) = *index_grid(&mut grid, x+dx, y+dy, z+dz, w+dw) {
                            merge(index, other, &mut map);
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", map);
    let result = num_stars - map.len();
    println!("{}", result);
}
