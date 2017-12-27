//! # [Day 18: Duet](http://adventofcode.com/2017/day/18)
//!
//! You discover a tablet containing some strange assembly code labeled
//! simply "[Duet]". Rather than bother the sound card with it, you decide
//! to run the code yourself. Unfortunately, you don't see any
//! documentation, so you're left to figure out what the instructions mean
//! on your own.
//!
//!   [Duet]: https://en.wikipedia.org/wiki/Duet

use std::collections::{ HashMap, VecDeque };
use nom::anychar;
use ::parse::signed_number;

/// Many of the instructions can take either a register (a single letter) or
/// a number. The value of a register is the integer it contains; the value
/// of a number is that number.
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

/// There aren't that many instructions, so it shouldn't be hard to figure
/// out what they do. Here's what you determine:
///
/// -   `snd X` *<span title="I don't recommend actually trying this.">plays
///     a sound</span>* with a frequency equal to the value of `X`.
/// -   `set X Y` *sets* register `X` to the value of `Y`.
/// -   `add X Y` *increases* register `X` by the value of `Y`.
/// -   `mul X Y` sets register `X` to the result of *multiplying* the value
///     contained in register `X` by the value of `Y`.
/// -   `mod X Y` sets register `X` to the *remainder* of dividing the value
///     contained in register `X` by the value of `Y` (that is, it sets `X`
///     to the result of `X` [modulo] `Y`).
/// -   `rcv X` *recovers* the frequency of the last sound played, but only
///     when the value of `X` is not zero. (If it is zero, the command does
///     nothing.)
/// -   `jgz X Y` *jumps* with an offset of the value of `Y`, but only if
///     the value of `X` is *greater than zero*. (An offset of `2` skips the
///     next instruction, an offset of `-1` jumps to the previous
///     instruction, and so on.)
///
///   [modulo]: https://en.wikipedia.org/wiki/Modulo_operation
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Instruction {
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value),
}

use self::Instruction::*;

impl Instruction {
    named!{ parse_snd (&[u8]) -> Instruction,
        do_parse!(
            tag!("snd ") >>
            value: call!(Value::from_bytes) >>

            (Snd(value))
        )
    }

    named!{ parse_set (&[u8]) -> Instruction,
        do_parse!(
            tag!("set ") >>
            reg: anychar >>
            char!(' ') >>
            value: call!(Value::from_bytes) >>

            (Set(reg, value))
        )
    }

    named!{ parse_add (&[u8]) -> Instruction,
        do_parse!(
            tag!("add ") >>
            reg: anychar >>
            char!(' ') >>
            value: call!(Value::from_bytes) >>

            (Add(reg, value))
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

    named!{ parse_mod (&[u8]) -> Instruction,
        do_parse!(
            tag!("mod ") >>
            reg: anychar >>
            char!(' ') >>
            value: call!(Value::from_bytes) >>

            (Mod(reg, value))
        )
    }

    named!{ parse_rcv (&[u8]) -> Instruction,
        do_parse!(
            tag!("rcv ") >>
            reg: anychar >>

            (Rcv(reg))
        )
    }

    named!{ parse_jgz (&[u8]) -> Instruction,
        do_parse!(
            tag!("jgz ") >>
            a: call!(Value::from_bytes) >>
            char!(' ') >>
            b: call!(Value::from_bytes) >>

            (Jgz(a, b))
        )
    }

    named!{ pub from_bytes (&[u8]) -> Instruction,
        alt!( call!(Instruction::parse_snd)
            | call!(Instruction::parse_set)
            | call!(Instruction::parse_add)
            | call!(Instruction::parse_mul)
            | call!(Instruction::parse_mod)
            | call!(Instruction::parse_rcv)
            | call!(Instruction::parse_jgz)
        )
    }

    named!{ pub list_from_bytes (&[u8]) -> Vec<Instruction>,
        lines!(Instruction::from_bytes)
    }
}

/// It seems like the assembly is meant to operate on a set of *registers*
///that are each named with a single letter and that can each hold a single
/// [integer]. You suppose each register should start with a value of `0`.
///
///   [integer]: https://en.wikipedia.org/wiki/Integer
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

/// After each *jump* instruction, the program continues with the
/// instruction to which the *jump* jumped. After any other instruction, the
/// program continues with the next instruction. Continuing (or jumping) off
/// either end of the program terminates it.
enum State {
    Snd(isize),
    Rcv(char),
    Continue,
    Terminate,
}

impl<'a> Computer<'a> {
    fn step(&mut self, other_sent: &mut VecDeque<isize>) -> State {
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
        match val {
            Literal(n) => n,
            Register(c) => *self.registers.entry(c).or_insert(0),
        }
    }
}

/// For example:
///
/// ```text
/// set a 1
/// add a 2
/// mul a a
/// mod a 5
/// snd a
/// set a 0
/// rcv a
/// jgz a -1
/// set a 1
/// jgz a -2
/// ```
///
/// -   The first four instructions set `a` to `1`, add `2` to it, square
///     it, and then set it to itself modulo `5`, resulting in a value of
///     `4`.
/// -   Then, a sound with frequency `4` (the value of `a`) is played.
/// -   After that, `a` is set to `0`, causing the subsequent `rcv` and
///     `jgz` instructions to both be skipped (`rcv` because `a` is `0`, and
///     `jgz` because `a` is not greater than `0`).
/// -   Finally, `a` is set to `1`, causing the next `jgz` instruction to
///     activate, jumping back two instructions to another jump, which jumps
///     again to the `rcv`, which ultimately triggers the *recover*
///     operation.
///
/// At the time the *recover* operation is executed, the frequency of the
/// last sound played is `4`.
///
/// ```
/// # use advent_solutions::advent2017::day18::{ Instruction, part1 };
/// # let input = "set a 1
/// # add a 2
/// # mul a a
/// # mod a 5
/// # snd a
/// # set a 0
/// # rcv a
/// # jgz a -1
/// # set a 1
/// # jgz a -2
/// # ";
/// let instructions = Instruction::list_from_bytes(input.as_bytes())
///     .to_full_result()
///     .expect("Error parsing instructions");
///
/// assert_eq!(part1(&instructions), 4);
/// ```
///
/// *What is the value of the recovered frequency* (the value of the most
/// recently played sound) the *first* time a `rcv` instruction is executed
/// with a non-zero value?
pub fn part1(instructions: &[Instruction]) -> isize {
    let mut computer = Computer::new(0, instructions);
    let mut frequency = None;
    let mut dummy_msg_q = VecDeque::new();

    loop {
        match computer.step(&mut dummy_msg_q) {
            State::Snd(v) => frequency = Some(v),
            State::Rcv(r) => if *computer.registers.entry(r).or_insert(0) != 0 {
                return frequency.expect("Recovered frequency with no sound played");
            } else {
                computer.ip += 1;
            },
            State::Continue => {},
            State::Terminate => panic!("Execution terminated with no recovered frequency"),
        }
    }
}

/// As you congratulate yourself for a job well done, you notice that the
/// documentation has been on the back of the tablet this entire time. While
/// you actually got most of the instructions correct, there are a few key
/// differences. This assembly code isn't about sound at all - it's meant to
/// be run *twice at the same time*.
///
/// Each running copy of the program has its own set of registers and
/// follows the code independently - in fact, the programs don't even
/// necessarily run at the same speed. To coordinate, they use the *send*
/// (`snd`) and *receive* (`rcv`) instructions:
///
/// -   `snd X` *sends* the value of `X` to the other program. These values
///     wait in a queue until that program is ready to receive them. Each
///     program has its own message queue, so a program can never receive a
///     message it sent.
/// -   `rcv X` *receives* the next value and stores it in register `X`. If
///     no values are in the queue, the program *waits for a value to be
///     sent to it*. Programs do not continue to the next instruction until
///     they have received a value. Values are received in the order they
///     are sent.
///
/// Each program also has its own *program ID* (one `0` and the other `1`);
/// the register `p` should begin with this value.
///
/// For example:
///
/// ```text
/// snd 1
/// snd 2
/// snd p
/// rcv a
/// rcv b
/// rcv c
/// rcv d
/// ```
///
/// Both programs begin by sending three values to the other. Program `0`
/// sends `1, 2, 0`; program `1` sends `1, 2, 1`. Then, each program
/// receives a value (both `1`) and stores it in `a`, receives another value
/// (both `2`) and stores it in `b`, and then each receives the program ID
/// of the other program (program `0` receives `1`; program `1` receives
/// `0`) and stores it in `c`. Each program now sees a different value in
/// its own copy of register `c`.
///
/// Finally, both programs try to `rcv` a *fourth* time, but no data is
/// waiting for either of them, and they reach a *deadlock*. When this
/// happens, both programs terminate.
///
/// ```
/// # use advent_solutions::advent2017::day18::{ Instruction, part2 };
/// # let input = "set a 1
/// # snd 1
/// # snd 2
/// # snd p
/// # rcv a
/// # rcv b
/// # rcv c
/// # rcv d
/// # ";
/// let instructions = Instruction::list_from_bytes(input.as_bytes())
///     .to_full_result()
///     .expect("Error parsing instructions");
///
/// assert_eq!(part2(&instructions), 3);
/// ```
///
/// It should be noted that it would be equally valid for the programs to
/// run at different speeds; for example, program `0` might have sent all
/// three values and then stopped at the first `rcv` before program `1`
/// executed even its first instruction.
///
/// Once both of your programs have terminated (regardless of what caused
/// them to do so), *how many times did program `1` send a value*?
pub fn part2(instructions: &[Instruction]) -> usize {
    let mut c0 = Computer::new(0, instructions);
    let mut c1 = Computer::new(1, instructions);

    let mut c0_sent = VecDeque::new();
    let mut c1_sent = VecDeque::new();

    let mut c0_running = true;
    let mut c1_running = true;

    let mut c1_total_sent = 0;

    while c0_running || c1_running {
        while c0_running {
            match c0.step(&mut c1_sent) {
                State::Snd(v) => c0_sent.push_back(v),
                State::Rcv(_) => {
                    if !c1_running  {
                        c0_running = false;
                    }

                    break;
                },
                State::Continue => {},
                State::Terminate => c0_running = false,
            }
        }

        while c1_running {
            match c1.step(&mut c0_sent) {
                State::Snd(v) => {
                    c1_total_sent += 1;
                    c1_sent.push_back(v);
                },
                State::Rcv(_) => {
                    if !c0_running || c1_sent.len() == 0 {
                        c1_running = false;
                    }

                    break;
                },
                State::Continue => {},
                State::Terminate => c1_running = false,
            }
        }
    }

    c1_total_sent
}


pub fn parse_input(input: &str) -> Vec<Instruction> {
    Instruction::list_from_bytes(input.as_bytes())
        .to_full_result()
        .expect("Error parsing instructions")
}

test_day!("18", 3423, 7493);
