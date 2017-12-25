extern crate advent;
#[macro_use] extern crate nom;

use std::collections::{ HashMap, VecDeque };
use std::str::FromStr;
use nom::{ anychar, digit };

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Value {
    Literal(isize),
    Register(char),
}

impl From<isize> for Value {
    fn from(x: isize) -> Self {
        Value::Literal(x)
    }
}

impl From<char> for Value {
    fn from(x: char) -> Self {
        Value::Register(x)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Instruction {
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value),
}

named!{ parse_digits (&[u8]) -> isize,
    map_res!(
        map_res!(digit, std::str::from_utf8),
        FromStr::from_str
    )
}

named!{ parse_number (&[u8]) -> isize,
    do_parse!(
        negative: opt!(char!('-')) >>
        digits: parse_digits >>

        (digits * if negative.is_some() { -1 } else { 1 })
    )
}

named!{ parse_value_literal (&[u8]) -> Value,
    map!(parse_number, Into::<Value>::into)
}

named!{ parse_value_register (&[u8]) -> Value,
    map!(anychar, Into::<Value>::into)
}

named!{ parse_value (&[u8]) -> Value,
    alt!( parse_value_literal | parse_value_register )
}

named!{ parse_snd (&[u8]) -> Instruction,
    do_parse!(
        tag!("snd ") >>
        value: parse_value >>

        (Instruction::Snd(value))
    )
}

named!{ parse_set (&[u8]) -> Instruction,
    do_parse!(
        tag!("set ") >>
        reg: anychar >>
        char!(' ') >>
        value: parse_value >>

        (Instruction::Set(reg, value))
    )
}

named!{ parse_add (&[u8]) -> Instruction,
    do_parse!(
        tag!("add ") >>
        reg: anychar >>
        char!(' ') >>
        value: parse_value >>

        (Instruction::Add(reg, value))
    )
}

named!{ parse_mul (&[u8]) -> Instruction,
    do_parse!(
        tag!("mul ") >>
        reg: anychar >>
        char!(' ') >>
        value: parse_value >>

        (Instruction::Mul(reg, value))
    )
}

named!{ parse_mod (&[u8]) -> Instruction,
    do_parse!(
        tag!("mod ") >>
        reg: anychar >>
        char!(' ') >>
        value: parse_value >>

        (Instruction::Mod(reg, value))
    )
}

named!{ parse_rcv (&[u8]) -> Instruction,
    do_parse!(
        tag!("rcv ") >>
        reg: anychar >>

        (Instruction::Rcv(reg))
    )
}

named!{ parse_jgz (&[u8]) -> Instruction,
    do_parse!(
        tag!("jgz ") >>
        a: parse_value >>
        char!(' ') >>
        b: parse_value >>

        (Instruction::Jgz(a, b))
    )
}

named!{ parse_instruction (&[u8]) -> Instruction,
    alt!(
        parse_snd
        | parse_set
        | parse_add
        | parse_mul
        | parse_mod
        | parse_rcv
        | parse_jgz
    )
}

named!{ parse_instructions (&[u8]) -> Vec<Instruction>,
    separated_list_complete!(tag!("\n"), parse_instruction)
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Computer<'a> {
    code: &'a [Instruction],
    ip: usize,
    registers: HashMap<char, isize>,
}

impl<'a> Computer<'a> {
    fn new(id: isize, code: &'a [Instruction]) -> Computer<'a> {
        let mut registers = HashMap::new();
        registers.insert('p', id);

        Computer { code, ip: 0, registers }
    }
}

enum State {
    Snd(isize),
    Rcv(char),
    Continue,
    Terminate,
}

impl<'a> Computer<'a> {
    fn step(&mut self, other_sent: &mut VecDeque<isize>) -> State {
        use Instruction::*;

        if self.ip >= self.code.len() {
            return State::Terminate;
        }

        match self.code[self.ip] {
            Snd(v) => {
                self.ip += 1;
                return State::Snd(self.get_value(v));
            },
            Set(r, v) => {
                *self.registers.entry(r).or_insert(0) = self.get_value(v);
                self.ip += 1;
            },
            Add(r, v) => {
                *self.registers.entry(r).or_insert(0) += self.get_value(v);
                self.ip += 1;
            },
            Mul(r, v) => {
                *self.registers.entry(r).or_insert(0) *= self.get_value(v);
                self.ip += 1;
            },
            Mod(r, v) => {
                *self.registers.entry(r).or_insert(0) %= self.get_value(v);
                self.ip += 1;
            },
            Rcv(r) => {
                if let Some(v) = other_sent.pop_front() {
                    *self.registers.entry(r).or_insert(0) = v;
                    self.ip += 1;
                } else {
                    return State::Rcv(r);
                }
            },
            Jgz(x, y) => {
                if self.get_value(x) > 0 {
                    self.ip = ((self.ip as isize) + self.get_value(y)) as usize;
                } else {
                    self.ip += 1;
                }
            },
        };

        State::Continue
    }

    fn get_value(&mut self, val: Value) -> isize {
        use Value::*;

        match val {
            Literal(n) => n,
            Register(c) => *self.registers.entry(c).or_insert(0),
        }
    }
}

fn step1(instructions: &[Instruction]) -> Option<isize> {
    let mut computer = Computer::new(0, instructions);
    let mut frequency = None;
    let mut dummy_msg_q = VecDeque::new();

    loop {
        use State::*;

        match computer.step(&mut dummy_msg_q) {
            Snd(v) => frequency = Some(v),
            Rcv(r) => if *computer.registers.entry(r).or_insert(0) != 0 {
                return frequency;
            },
            Continue => {},
            Terminate => return None,
        }
    }
}

fn step2(instructions: &[Instruction]) -> usize {
    let mut c0 = Computer::new(0, instructions);
    let mut c1 = Computer::new(1, instructions);

    let mut c0_sent = VecDeque::new();
    let mut c1_sent = VecDeque::new();

    let mut c0_running = true;
    let mut c1_running = true;

    let mut c1_total_sent = 0;

    while c0_running || c1_running {
        use State::*;

        while c0_running {
            match c0.step(&mut c1_sent) {
                Snd(v) => c0_sent.push_back(v),
                Rcv(_) => {
                    if !c1_running  {
                        c0_running = false;
                    }

                    break;
                },
                Continue => {},
                Terminate => c0_running = false,
            }
        }

        while c1_running {
            match c1.step(&mut c0_sent) {
                Snd(v) => {
                    c1_total_sent += 1;
                    c1_sent.push_back(v);
                },
                Rcv(_) => {
                    if !c0_running || c1_sent.len() == 0 {
                        c1_running = false;
                    }

                    break;
                },
                Continue => {},
                Terminate => c1_running = false,
            }
        }
    }

    c1_total_sent
}

fn main() {
    let input = advent::download_input(2017, 18);

    let instructions = parse_instructions(input.as_bytes())
        .to_full_result()
        .expect("Error parsing instructions");

    println!("Step 1: {}", step1(&instructions).expect("Could not find frequency"));
    println!("Step 2: {}", step2(&instructions));
}
