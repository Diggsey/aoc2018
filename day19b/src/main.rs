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
    registers[0] = 1;
    let lines: Vec<_> = line_iter.map(|line| {
        let (op, a, b, c): (String, usize, usize, usize);
        scan!(line.bytes() => "{} {} {} {}", op, a, b, c);
        (OpCode::parse(&op), a, b, c)
    }).collect();

    for _ in 0..10 {
        let (op, a, b, c) = lines[registers[ip_reg]];
        op.apply(&mut registers, a, b, c);
        registers[ip_reg] += 1;
    }

    // Reverse engineering the program shows that it generates a target value in register 5,
    // and then adds together all of the factors of that target value to produce its result.
    println!("TARGET: {}", registers[5]);

    let factors_of_target = [
        1, 2, 5, 10, 25, 41, 50, 82, 205, 410, 1025, 2050, 5147,
        10294, 25735, 51470, 128675, 211027, 257350, 422054,
        1055135, 2110270, 5275675, 10551350
    ];
    println!("FACTORS OF TARGET: {:?}", factors_of_target);
    println!("SUM OF FACTORS: {}", factors_of_target.iter().sum::<u64>());
}
