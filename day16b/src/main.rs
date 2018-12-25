use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap};

use itertools::Itertools;

use self::OpCode::*;

#[macro_use]
extern crate text_io;

type Reg = usize;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum OpCode {
    AddR, AddI, MulR, MulI, BanR, BanI, BorR, BorI,
    SetR, SetI, GtIR, GtRI, GtRR, EqIR, EqRI, EqRR,
}

impl OpCode {
    fn apply(self, registers: &mut [Reg], a: usize, b: usize, c: usize) {
        registers[c] = match self {
            AddR => registers[a] + registers[b],
            AddI => registers[a] + b,
            MulR => registers[a] * registers[b],
            MulI => registers[a] * b,
            BanR => registers[a] & registers[b],
            BanI => registers[a] & b,
            BorR => registers[a] | registers[b],
            BorI => registers[a] | b,
            SetR => registers[a],
            SetI => a,
            GtIR => (a > registers[b]).into(),
            GtRI => (registers[a] > b).into(),
            GtRR => (registers[a] > registers[b]).into(),
            EqIR => (a == registers[b]).into(),
            EqRI => (registers[a] == b).into(),
            EqRR => (registers[a] == registers[b]).into(),
        }
    }
}

const ALL_OP_CODES: &[OpCode] = &[
    AddR, AddI, MulR, MulI, BanR, BanI, BorR, BorI,
    SetR, SetI, GtIR, GtRI, GtRR, EqIR, EqRI, EqRR,
];

fn main() {
    let mut line_iter = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap);
    
    let mut op_codes = vec![ALL_OP_CODES.iter().cloned().collect::<HashSet<OpCode>>(); 16];
    let mut known_codes = HashMap::new();
    
    for (l1, l2, l3, _) in line_iter.by_ref().scan(0, |state, line| {
        if line.is_empty() { *state += 1; } else { *state = 0; }
        if *state >= 2 { None } else { Some(line) }
    }).tuples() {
        let (r0, r1, r2, r3): (Reg, Reg, Reg, Reg);
        let (s0, s1, s2, s3): (Reg, Reg, Reg, Reg);
        let (op, a, b, c): (usize, usize, usize, usize);
        scan!(l1.bytes() => "Before: [{}, {}, {}, {}]", r0, r1, r2, r3);
        scan!(l2.bytes() => "{} {} {} {}", op, a, b, c);
        scan!(l3.bytes() => "After:  [{}, {}, {}, {}]", s0, s1, s2, s3);

        let possible: HashSet<OpCode> = ALL_OP_CODES.iter().cloned().filter(|op_code| {
            let mut registers = [r0, r1, r2, r3];
            op_code.apply(&mut registers, a, b, c);
            registers == [s0, s1, s2, s3]
        }).collect();

        op_codes[op].retain(|op_code| possible.contains(&op_code));
    }

    while known_codes.len() < ALL_OP_CODES.len() {
        for (code, ops) in op_codes.iter().enumerate() {
            if ops.len() == 1 && !known_codes.contains_key(&code) {
                known_codes.insert(code, ops.iter().cloned().next().unwrap());
            }
        }
        for (&code, &op) in known_codes.iter() {
            for (code2, ops) in op_codes.iter_mut().enumerate() {
                if code2 != code {
                    ops.remove(&op);
                }
            }
        }
    }
    println!("{:?}", known_codes);

    let mut registers = [0,0,0,0];
    for line in line_iter {
        if line.is_empty() { continue; }
        let (op, a, b, c): (usize, usize, usize, usize);
        scan!(line.bytes() => "{} {} {} {}", op, a, b, c);
        known_codes[&op].apply(&mut registers, a, b, c);
    }
    println!("{:?}", registers);
}
