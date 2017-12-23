extern crate advent;
#[macro_use] extern crate nom;

use std::str::FromStr;
use nom::{ alpha, digit };

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Action { Inc, Dec }

impl<'a> FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Action::*;

        match s {
            "inc" => Ok(Inc),
            "dec" => Ok(Dec),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Cmp { LT, LTE, EQ, NE, GTE, GT }

impl Cmp {
    fn compare<T: Ord + PartialOrd>(&self, a: T, b: T) -> bool {
        use Cmp::*;

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
        use Cmp::*;

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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Instruction<'a> {
    register: &'a str,
    action: Action,
    amount: isize,
    condition_reg: &'a str,
    condition_cmp: Cmp,
    condition_amt: isize,
}

named!{ parse_register (&[u8]) -> &str,
    map_res!(alpha, std::str::from_utf8)
}

named!{ parse_amount (&[u8]) -> isize,
    do_parse!(
        negative: opt!(char!('-')) >>
        digits: map_res!(digit, std::str::from_utf8) >>

        (digits.parse::<isize>().unwrap()
            * if negative.is_some() { -1 } else { 1 })
    )
}

named!{ parse_instruction (&[u8]) -> Instruction,
    do_parse!(
        register: parse_register >>

        tag!(" ") >>

        action: map_res!(
            map_res!(
                alt!(tag!("inc") | tag!("dec")),
                std::str::from_utf8
            ),
            FromStr::from_str
        ) >>

        tag!(" ") >>

        amount: parse_amount >>

        tag!(" if ") >>

        condition_reg: parse_register >>

        tag!(" ") >>

        condition_cmp: map_res!(
            map_res!(
                is_a!(&[b'<', b'=', b'>', b'!'][..]),
                std::str::from_utf8
            ),
            FromStr::from_str
        ) >>

        tag!(" ") >>

        condition_amt: parse_amount >>

        tag!("\n") >>

        (Instruction { register, action, amount, condition_reg, condition_cmp, condition_amt })
    )
}

named!{ parse_instructions (&[u8]) -> Vec<Instruction>,
    many0!(parse_instruction)
}

fn main() {
    let input = advent::download_input(2017, 8);

    let instructions = parse_instructions(input.as_bytes())
        .to_full_result()
        .expect("Error parsing instructions");

    use std::collections::HashMap;

    let mut registers: HashMap<&str, isize> = HashMap::new();

    let mut max_reg_value = isize::min_value();

    for &Instruction { register, action, amount, condition_reg, condition_cmp, condition_amt }
        in &instructions
    {
        let reg_value = *registers.get(condition_reg).unwrap_or(&0);

        if condition_cmp.compare(reg_value, condition_amt) {
            match action {
                Action::Inc => *registers.entry(register).or_insert(0) += amount,
                Action::Dec => *registers.entry(register).or_insert(0) -= amount,
            }
        }

        let new_reg_value = *registers.get(register).unwrap_or(&0);

        if new_reg_value > max_reg_value {
            max_reg_value = new_reg_value;
        }
    }

    let step1 = registers.values().max().expect("No registers found?");

    println!("Step 1: {}", step1);
    println!("Step 2: {}", max_reg_value);
}
