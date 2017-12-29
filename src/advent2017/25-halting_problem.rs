use std::collections::HashMap;
use nom::anychar;
use ::parse::unsigned_number;

named!{ parse_bool (&[u8]) -> bool,
    map!(
        alt!(char!('0') | char!('1')),
        |x| x == '1'
    )
}

named!{ parse_right (&[u8]) -> bool,
    map!(
        alt!(tag!("left") | tag!("right")),
        |x| x == b"right"
    )
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct State {
    false_write: bool,
    false_right: bool,
    false_next: char,
    true_write: bool,
    true_right: bool,
    true_next: char,
}

impl State {
    named!{ from_bytes (&[u8]) -> (char, State),
        do_parse!(
            tag!("In state ") >>
            name: anychar >>
            tag!(":
  If the current value is 0:
    - Write the value ") >>
            false_write: parse_bool >>
            tag!(".
    - Move one slot to the ") >>
            false_right: parse_right >>
            tag!(".
    - Continue with state ") >>
            false_next: anychar >>

            tag!(".
  If the current value is 1:
    - Write the value ") >>
            true_write: parse_bool >>
            tag!(".
    - Move one slot to the ") >>
            true_right: parse_right >>
            tag!(".
    - Continue with state ") >>
            true_next: anychar >>
            tag!(".
") >>

            ((
                name,
                State { false_write, false_right, false_next, true_write, true_right, true_next }
            ))
        )
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Blueprint {
    begin: char,
    checksum_after: usize,
    states: HashMap<char, State>,
}

impl Blueprint {
    named!{ from_bytes (&[u8]) -> Blueprint,
        do_parse!(
            tag!("Begin in state ") >>
            begin: anychar >>
            tag!(".
Perform a diagnostic checksum after ") >>
            checksum_after: unsigned_number >>
            tag!(" steps.

") >>
            states: map!(
                lines!(call!(State::from_bytes)),
                |vec| vec.into_iter().collect::<HashMap<_, _>>()
            ) >>

            (Blueprint { begin, checksum_after, states })
        )
    }
}

pub fn part1(input: &Blueprint) -> usize {
    0
}

pub fn part2(_: &Blueprint) -> usize {
    0
}

pub fn parse_input(input: &str) -> Blueprint {
    Blueprint::from_bytes(input.as_bytes())
        .to_full_result()
        .expect("Error parsing blueprint")
}

test_day!("25", 0, 0);
