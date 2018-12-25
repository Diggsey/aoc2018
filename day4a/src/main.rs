use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use std::collections::HashMap;

#[macro_use]
extern crate text_io;

#[derive(Debug, Clone, Copy)]
enum Event {
    BeginsShift { month: i32, day: i32, hour: i32, guard_id: i32 },
    FallsAsleep { month: i32, day: i32, minute: i32 },
    WakesUp { month: i32, day: i32, minute: i32 },
}

fn scan_begins_shift(line: &str) -> Result<Event, Box<Error>> {
    let (month, day, hour, minute, guard_id): (i32, i32, i32, i32, i32);
    try_scan!(line.bytes() => "[1518-{}-{} {}:{}] Guard #{} begins shift", month, day, hour, minute, guard_id);
    Ok(Event::BeginsShift { month, day, hour, guard_id })
}

fn scan_falls_asleep(line: &str) -> Result<Event, Box<Error>> {
    let (month, day, hour, minute): (i32, i32, i32, i32);
    try_scan!(line.bytes() => "[1518-{}-{} {}:{}] falls asleep", month, day, hour, minute);
    Ok(Event::FallsAsleep { month, day, minute })
}

fn scan_wakes_up(line: &str) -> Result<Event, Box<Error>> {
    let (month, day, hour, minute): (i32, i32, i32, i32);
    try_scan!(line.bytes() => "[1518-{}-{} {}:{}] wakes up", month, day, hour, minute);
    Ok(Event::WakesUp { month, day, minute })
}

fn scan_event(line: &str) -> Event {
    scan_begins_shift(line).or_else(|_| scan_falls_asleep(line)).or_else(|_| scan_wakes_up(line)).unwrap()
}

#[derive(Debug)]
struct Guard {
    days: i32,
    counts: Vec<i32>,
}

impl Default for Guard {
    fn default() -> Self {
        Guard { days: 0, counts: vec![0; 60]}
    }
}

fn days_in_month(month: i32) -> i32 {
    match month {
        1|3|5|7|8|10|12 => 31,
        4|6|9|11 => 30,
        2 => 28,
        _ => unreachable!()
    }
}

fn main() {
    let events: Vec<_> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|l| scan_event(&l))
        .collect();
    
    let mut days = HashMap::new();
    let mut guards: HashMap<i32, Guard> = HashMap::new();
    for event in events.iter().cloned() {
        if let Event::BeginsShift { mut month, mut day, hour, guard_id } = event {
            if hour > 12 { day += 1; }
            if day > days_in_month(month) { month += 1; day = 1; }
            days.insert((month, day), guard_id);
            guards.entry(guard_id).or_default().days += 1;
        }
    }

    for event in events {
        let (month, day, minute, inc) = match event {
            Event::FallsAsleep { month, day, minute } => (month, day, minute, 1),
            Event::WakesUp { month, day, minute } => (month, day, minute, -1),
            _ => continue
        };

        println!("{} {}", month, day);
        let guard_id = days[&(month, day)];
        let guard = guards.get_mut(&guard_id).unwrap();
        for m in minute..60 {
            guard.counts[m as usize] += inc;
        }
    }

    let (guard_id, guard) = guards.iter()
        .max_by_key(|(_, v)| -> i32 {
            v.counts.iter().sum::<i32>()
        }).unwrap();
    
    let minute = guard.counts.iter().enumerate().max_by_key(|(_, &v)| v).unwrap().0 as i32;
    
    println!("{} {} {:?}", guard_id, minute, guard);
    println!("{}", guard_id * minute);
}
