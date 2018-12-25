use std::fs::File;
use std::io::{BufReader, BufRead};

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
    fn parse(input: &str) -> Self {
        match input {
            "addr" => AddR, "addi" => AddI, "mulr" => MulR, "muli" => MulI,
            "banr" => BanR, "bani" => BanI, "borr" => BorR, "bori" => BorI,
            "setr" => SetR, "seti" => SetI, "gtir" => GtIR, "gtri" => GtRI,
            "gtrr" => GtRR, "eqir" => EqIR, "eqri" => EqRI, "eqrr" => EqRR,
            _ => unreachable!()
        }
    }
}

fn main() {
    let mut line_iter = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap);

    let ip_reg: usize;
    let first_line = line_iter.next().unwrap();
    scan!(first_line.bytes() => "#ip {}", ip_reg);

    let mut registers = [0; 6];
    let lines: Vec<_> = line_iter.map(|line| {
        let (op, a, b, c): (String, usize, usize, usize);
        scan!(line.bytes() => "{} {} {} {}", op, a, b, c);
        (OpCode::parse(&op), a, b, c)
    }).collect();

    while registers[ip_reg] < lines.len() {
        let (op, a, b, c) = lines[registers[ip_reg]];
        op.apply(&mut registers, a, b, c);
        registers[ip_reg] += 1;
    }

    println!("{:?}", registers);
}
