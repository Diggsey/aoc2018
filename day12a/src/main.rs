use std::fs::File;
use std::io::{BufReader, BufRead};

#[macro_use]
extern crate text_io;

fn rule_index(bits: &[bool]) -> usize {
    let mut result = 0;
    for &bit in bits {
        result <<= 1;
        if bit { result |= 1; }
    }
    result
}

fn is_set(c: char) -> bool {
    c == '#'
}

fn main() {
    let mut lines = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap);

    let initial_state_line = lines.next().unwrap();
    let initial_state_str: String;
    scan!(initial_state_line.bytes() => "initial state: {}", initial_state_str);

    let mut state: Vec<_> = initial_state_str.chars().map(is_set).collect();
    let mut state_offset = 0;

    let mut rules = [false; 32];
    lines.next();
    for line in lines {
        let pattern_str: String;
        let result_str: String;
        scan!(line.bytes() => "{} => {}", pattern_str, result_str);
        let rule: Vec<_> = pattern_str.chars().map(is_set).collect();
        rules[rule_index(&rule)] = is_set(result_str.chars().next().unwrap());
    }

    for i in 0..20 {
        if state[0] { state.insert(0, false); state_offset -= 1; }
        if state[1] { state.insert(0, false); state_offset -= 1; }
        if state[2] { state.insert(0, false); state_offset -= 1; }
        if state[3] { state.insert(0, false); state_offset -= 1; }
        if state[state.len()-1] { state.push(false); }
        if state[state.len()-2] { state.push(false); }
        if state[state.len()-3] { state.push(false); }
        if state[state.len()-4] { state.push(false); }
        let mut state2 = vec![false; state.len()];

        for x in 2..(state.len()-2) {
            state2[x] = rules[rule_index(&state[(x-2)..=(x+2)])];
        }

        state = state2;

        let display: String = state.iter().map(|&b| if b { '#' } else { '.' }).collect();
        println!("{}: {}", i, display);
    }

    let result: i32 = state.iter().enumerate().filter(|&(_, &v)| v).map(|(i, _)| i as i32 + state_offset).sum();
    println!("{}", result);
}
