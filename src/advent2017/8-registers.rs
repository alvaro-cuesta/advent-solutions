//! # [Day 8: I Heard You Like Registers](http://adventofcode.com/2017/day/8)
//!
//! You receive a <span title="There's that sorcery I told you about.">signal
//! </span> directly from the CPU. Because of your recent assistance with [jump
//! instructions], it would like you to compute the result of a series of
//! unusual register instructions.
//!
//!   [jump instructions]: ../day5/index.html

use std::collections::HashMap;
use std::str::FromStr;
use ::parse::{ name as parse_name, signed_number as parse_signed_number };

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Action { Inc, Dec }

use self::Action::*;

impl<'a> FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(Inc),
            "dec" => Ok(Dec),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Cmp { LT, LTE, EQ, NE, GTE, GT }

use self::Cmp::*;

impl Cmp {
    fn compare<T: Ord + PartialOrd>(&self, a: T, b: T) -> bool {
        match *self {
            LT => a < b,
            LTE => a <= b,
            EQ => a == b,
            NE => a != b,
            GTE => a >= b,
            GT => a > b,
        }
    }
}

impl FromStr for Cmp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(LT),
            "<=" => Ok(LTE),
            "==" => Ok(EQ),
            "!=" => Ok(NE),
            ">=" => Ok(GTE),
            ">" => Ok(GT),
            _ => Err(()),
        }
    }
}

/// Each instruction consists of several parts: the register to modify,
/// whether to increase or decrease that register's value, the amount by
/// which to increase or decrease it, and a condition. If the condition
/// fails, skip the instruction without modifying the register. The
/// registers all start at `0`. The instructions look like this:
///
/// ```text
/// b inc 5 if a > 1
/// a inc 1 if b < 5
/// c dec -10 if a >= 1
/// c inc -20 if c == 10
/// ```
///
/// These instructions would be processed as follows:
///
/// -   Because `a` starts at `0`, it is not greater than `1`, and so `b` is
///     not modified.
/// -   `a` is increased by `1` (to `1`) because `b` is less than `5` (it is
///     `0`).
/// -   `c` is decreased by `-10` (to `10`) because `a` is now greater than
///     or equal to `1` (it is `1`).
/// -   `c` is increased by `-20` (to `-10`) because `c` is equal to `10`.
///
/// After this process, the largest value in any register is `1`.
///
/// You might also encounter `<=` (less than or equal to) or `!=` (not equal
/// to). However, the CPU doesn't have the bandwidth to tell you what all
/// the registers are named, and leaves that to you to determine.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Instruction<'a> {
    register: &'a str,
    action: Action,
    amount: isize,
    condition_reg: &'a str,
    condition_cmp: Cmp,
    condition_amt: isize,
}

impl<'a> Instruction<'a> {
    named!{ pub from_bytes (&[u8]) -> Instruction,
        do_parse!(
            register: parse_name >>
            char!(' ') >>
            action: from_str_bytes!(alt!(tag!("inc") | tag!("dec"))) >>
            char!(' ') >>
            amount: parse_signed_number >>
            tag!(" if ") >>
            condition_reg: parse_name >>
            char!(' ') >>
            condition_cmp: from_str_bytes!(is_a!(&[b'<', b'=', b'>', b'!'][..])) >>
            char!(' ') >>
            condition_amt: parse_signed_number >>

            (Instruction { register, action, amount, condition_reg, condition_cmp, condition_amt })
        )
    }

    named!{ pub list_from_bytes(&[u8]) -> Vec<Instruction>,
        lines!(Instruction::from_bytes)
    }

    pub fn exec(&self, registers: &mut HashMap<&'a str, isize>) {
        let reg_value = *registers.get(self.condition_reg).unwrap_or(&0);

        if self.condition_cmp.compare(reg_value, self.condition_amt) {
            match self.action {
                Action::Inc => *registers.entry(self.register).or_insert(0) += self.amount,
                Action::Dec => *registers.entry(self.register).or_insert(0) -= self.amount,
            }
        }
    }
}

/// *What is the largest value in any register* after completing the
/// instructions in your puzzle input?
///
/// ```text
/// b inc 5 if a > 1
/// a inc 1 if b < 5
/// c dec -10 if a >= 1
/// c inc -20 if c == 10
/// ```
///
/// After this process, the largest value in any register is `1`.
///
/// ```
/// # use advent_solutions::advent2017::day8::{ Instruction, part1 };
/// # let input = b"b inc 5 if a > 1
/// # a inc 1 if b < 5
/// # c dec -10 if a >= 1
/// # c inc -20 if c == 10
/// # ";
/// let instructions = Instruction::list_from_bytes(input)
///     .to_result()
///     .unwrap();
///
/// assert_eq!(part1(&instructions), 1);
/// ```
pub fn part1(instructions: &[Instruction]) -> isize {
    let mut registers: HashMap<&str, isize> = HashMap::new();

    for instruction in instructions {
        instruction.exec(&mut registers);
    }

    registers.values().max().map(|x| *x).unwrap_or(0)
}

/// To be safe, the CPU also needs to know *the highest value held in any
/// register during this process* so that it can decide how much memory to
/// allocate to these operations. For example, in the above instructions,
/// the highest value ever held was `10` (in register `c` after the third
/// instruction was evaluated).
///
/// ```
/// # use advent_solutions::advent2017::day8::{ Instruction, part2 };
/// let input = b"b inc 5 if a > 1
/// a inc 1 if b < 5
/// c dec -10 if a >= 1
/// c inc -20 if c == 10
/// ";
///
/// let instructions = Instruction::list_from_bytes(input)
///     .to_result()
///     .unwrap();
///
/// assert_eq!(part2(&instructions), 10);
/// ```
pub fn part2(instructions: &[Instruction]) -> isize {
    let mut registers: HashMap<&str, isize> = HashMap::new();

    let mut max_reg_value = isize::min_value();

    for instruction in instructions {
        instruction.exec(&mut registers);

        let new_reg_value = *registers.get(instruction.register).unwrap_or(&0);

        if new_reg_value > max_reg_value {
            max_reg_value = new_reg_value;
        }
    }

    max_reg_value
}

pub fn main() {
    let input = ::download::input(2017, 8);

    let instructions = Instruction::list_from_bytes(input.as_bytes())
        .to_full_result()
        .expect("Error parsing instructions");

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
}
