const SERIAL: i32 = 5177;

fn power_level((x, y): (i32, i32)) -> i32 {
    let rack_id = (x+1)+10;
    (((rack_id*(y+1) + SERIAL)*rack_id/100) % 10) - 5
}

fn main() {
    let (p, coord) = (0..(300-2))
        .flat_map(|y| (0..(300-2)).map(move |x| (x, y)))
        .map(|(x, y)| {
            ((0..3).flat_map(|dy| (0..3).map(move |dx| (x+dx, y+dy))).map(power_level).sum::<i32>(), (x, y))
        })
        .max_by_key(|&(p, _)| p)
        .unwrap();
    println!("{} ({},{})", p, coord.0+1, coord.1+1);
}
