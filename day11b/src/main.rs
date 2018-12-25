const SERIAL: i32 = 5177;

fn power_level((x, y): (i32, i32)) -> i32 {
    let rack_id = (x+1)+10;
    (((rack_id*(y+1) + SERIAL)*rack_id/100) % 10) - 5
}

fn main() {
    let mut grid = vec![[0; 300]; 300];
    for y in 0..300 {
        for x in 0..300 {
            grid[y][x] = power_level((x as i32, y as i32));
        }
    }
    let grid = &grid;

    let (p, coord) = (1..300)
        .flat_map(|s| (0..=(300-s)).map(move |y| (y, s)))
        .flat_map(|(y, s)| (0..=(300-s)).map(move |x| (x, y, s)))
        .map(|(x, y, s)| {
            ((0..s).flat_map(|dy| (0..s).map(move |dx| grid[y+dy][x+dx])).sum::<i32>(), (x, y, s))
        })
        .max_by_key(|&(p, _)| p)
        .unwrap();
    println!("{} ({},{},{})", p, coord.0+1, coord.1+1, coord.2);
}
