use std::collections::HashMap;
use nom::anychar;
use ::parse::signed_number;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Value {
    Literal(isize),
    Register(char),
}

use self::Value::*;

impl Value {
    named!{ parse_literal (&[u8]) -> Value,
        map!(signed_number, Into::<Value>::into)
    }

    named!{ parse_register (&[u8]) -> Value,
        map!(anychar, Into::<Value>::into)
    }

    named!{ pub from_bytes (&[u8]) -> Value,
        alt!( call!(Value::parse_literal) | call!(Value::parse_register) )
    }
}

impl From<isize> for Value {
    fn from(x: isize) -> Self {
        Literal(x)
    }
}

impl From<char> for Value {
    fn from(x: char) -> Self {
        Register(x)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Instruction {
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value),
}

use self::Instruction::*;

impl Instruction {
    named!{ parse_set (&[u8]) -> Instruction,
        do_parse!(
            tag!("set ") >>
            reg: anychar >>
            char!(' ') >>
            value: call!(Value::from_bytes) >>

            (Set(reg, value))
        )
    }

    named!{ parse_sub (&[u8]) -> Instruction,
        do_parse!(
            tag!("sub ") >>
            reg: anychar >>
            char!(' ') >>
            value: call!(Value::from_bytes) >>

            (Sub(reg, value))
        )
    }

    named!{ parse_mul (&[u8]) -> Instruction,
        do_parse!(
            tag!("mul ") >>
            reg: anychar >>
            char!(' ') >>
            value: call!(Value::from_bytes) >>

            (Mul(reg, value))
        )
    }

    named!{ parse_jnz (&[u8]) -> Instruction,
        do_parse!(
            tag!("jnz ") >>
            a: call!(Value::from_bytes) >>
            char!(' ') >>
            b: call!(Value::from_bytes) >>

            (Jnz(a, b))
        )
    }

    named!{ pub from_bytes (&[u8]) -> Instruction,
        alt!( call!(Instruction::parse_set)
            | call!(Instruction::parse_sub)
            | call!(Instruction::parse_mul)
            | call!(Instruction::parse_jnz)
        )
    }

    named!{ pub list_from_bytes (&[u8]) -> Vec<Instruction>,
        lines!(Instruction::from_bytes)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Computer<'a> {
    code: &'a [Instruction],
    ip: usize,
    registers: HashMap<char, isize>,
}

impl<'a> Computer<'a> {
    fn new(a: isize, code: &'a [Instruction]) -> Computer<'a> {
        let mut registers = HashMap::new();
        registers.insert('a', a);

        Computer { code, ip: 0, registers }
    }
}

enum State {
    Continue,
    Terminate,
    Mul,
}

impl<'a> Computer<'a> {
    fn step(&mut self) -> State {
        if self.ip >= self.code.len() {
            return State::Terminate;
        }

        match self.code[self.ip] {
            Set(r, v) => {
                *self.registers.entry(r).or_insert(0) = self.get_value(v);
                self.ip += 1;
            },
            Sub(r, v) => {
                *self.registers.entry(r).or_insert(0) -= self.get_value(v);
                self.ip += 1;
            },
            Mul(r, v) => {
                *self.registers.entry(r).or_insert(0) *= self.get_value(v);
                self.ip += 1;
                return State::Mul;
            },
            Jnz(x, y) => {
                if self.get_value(x) != 0 {
                    self.ip = ((self.ip as isize) + self.get_value(y)) as usize;
                } else {
                    self.ip += 1;
                }
            },
        };

        State::Continue
    }

    fn get_value(&mut self, val: Value) -> isize {
        match val {
            Literal(n) => n,
            Register(c) => *self.registers.entry(c).or_insert(0),
        }
    }
}

pub fn part1(instructions: &[Instruction]) -> isize {
    let mut computer = Computer::new(0, instructions);
    let mut mul_called = 0;

    loop {
        match computer.step() {
            State::Mul => mul_called += 1,
            State::Continue => {},
            State::Terminate => return mul_called,
        }
    }
}

pub fn is_prime(x: usize) -> bool {
    if x <= 1 { return false; }

    if x % 2 == 0 { return false; }

    for _ in (1..)
        .map(|n| 2*n+1)
        .take_while(|n| n*n < x)
        .filter(|n| x % n == 0)
    {
        return false;
    }

    true
}

pub fn part2(instructions: &[Instruction]) -> usize {
    if let Set('b', Literal(x)) = instructions[0] {
        if x < 0 { panic!("Unexpected negative number") }

        let x = x as usize;

        (0..1001)
            .map(|i| 100000 + x * 100 + i * 17)
            .filter(|&x| !is_prime(x))
            .count()
    } else {
        panic!("Expected first instruction to set b to a literal");
    }
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    Instruction::list_from_bytes(input.as_bytes())
        .to_full_result()
        .expect("Error parsing instructions")
}

test_day!("23", 6241, 909);
