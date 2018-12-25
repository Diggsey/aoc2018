use std::fs::File;
use std::io::{BufReader, BufRead};

use itertools::Itertools;

use self::OpCode::*;

#[macro_use]
extern crate text_io;

type Reg = usize;

#[derive(Copy, Clone, Debug)]
enum OpCode {
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
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
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
];

fn main() {
    let mut line_iter = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap);
    
    let result = line_iter.by_ref().scan(0, |state, line| {
        if line.is_empty() { *state += 1; } else { *state = 0; }
        if *state >= 2 { None } else { Some(line) }
    }).tuples().filter(|(l1, l2, l3, _)| {
        let (r0, r1, r2, r3): (Reg, Reg, Reg, Reg);
        let (s0, s1, s2, s3): (Reg, Reg, Reg, Reg);
        let (op, a, b, c): (usize, usize, usize, usize);
        scan!(l1.bytes() => "Before: [{}, {}, {}, {}]", r0, r1, r2, r3);
        scan!(l2.bytes() => "{} {} {} {}", op, a, b, c);
        scan!(l3.bytes() => "After:  [{}, {}, {}, {}]", s0, s1, s2, s3);

        ALL_OP_CODES.iter().filter(|op_code| {
            let mut registers = [r0, r1, r2, r3];
            op_code.apply(&mut registers, a, b, c);
            registers == [s0, s1, s2, s3]
        }).count() >= 3
    }).count();
    println!("{}", result);
}
