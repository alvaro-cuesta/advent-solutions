//! # [Day 25: The Halting Problem](http://adventofcode.com/2017/day/25)
//!
//! Following the twisty passageways deeper and deeper into the CPU, you
//! finally reach the <span title="Get it? CPU core?">core</span> of the
//! computer. Here, in the expansive central chamber, you find a grand
//! apparatus that fills the entire room, suspended nanometers above your
//! head.
//!
//! You had always imagined CPUs to be noisy, chaotic places, bustling with
//! activity. Instead, the room is quiet, motionless, and dark.
//!
//! Suddenly, you and the CPU's *garbage collector* startle each other.
//! "It's not often we get many visitors here!", he says. You inquire about
//! the stopped machinery.
//!
//! "It stopped milliseconds ago; not sure why. I'm a garbage collector, not
//! a doctor." You ask what the machine is for.
//!
//! "Programs these days, don't know their origins. That's the *Turing
//! machine*! It's what makes the whole computer work." You try to explain
//! that Turing machines are merely models of computation, but he cuts you
//! off. "No, see, that's just what they *want* you to think. Ultimately,
//! inside every CPU, there's a Turing machine driving the whole thing! Too
//! bad this one's broken. [We're doomed!]"
//!
//! You ask how you can help. "Well, unfortunately, the only way to get the
//! computer running again would be to create a whole new Turing machine
//! from scratch, but there's no *way* you can-" He notices the look on your
//! face, gives you a curious glance, shrugs, and goes back to sweeping the
//! floor.
//!
//!   [We're doomed!]: https://www.youtube.com/watch?v=cTwZZz0HV8I

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

/// You find the *Turing machine blueprints* (your puzzle input) on a tablet
/// in a nearby pile of debris. Looking back up at the broken Turing machine
/// above, you can start to identify its parts:
///
/// -   A *tape* which contains `0` repeated infinitely to the left and
///     right.
/// -   A *cursor*, which can move left or right along the tape and read or
///     write values at its current position.
/// -   A set of *states*, each containing rules about what to do based on
///     the current value under the cursor.
///
/// Each slot on the tape has two possible values: `0` (the starting value
/// for all slots) and `1`. Based on whether the cursor is pointing at a `0`
/// or a `1`, the current state says *what value to write* at the current
/// position of the cursor, whether to *move the cursor* left or right one
/// slot, and *which state to use next*.
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

/// For example, suppose you found the following blueprint:
///
/// ```text
/// Begin in state A.
/// Perform a diagnostic checksum after 6 steps.
///
/// In state A:
///   If the current value is 0:
///     - Write the value 1.
///     - Move one slot to the right.
///     - Continue with state B.
///   If the current value is 1:
///     - Write the value 0.
///     - Move one slot to the left.
///     - Continue with state B.
///
/// In state B:
///   If the current value is 0:
///     - Write the value 1.
///     - Move one slot to the left.
///     - Continue with state A.
///   If the current value is 1:
///     - Write the value 1.
///     - Move one slot to the right.
///     - Continue with state A.
/// ```
///
/// Running it until the number of steps required to take the listed
/// *diagnostic checksum* would result in the following tape configurations
/// (with the *cursor* marked in square brackets):
///
/// ```text
/// ... 0  0  0 [0] 0  0 ... (before any steps; about to run state A)
/// ... 0  0  0  1 [0] 0 ... (after 1 step;     about to run state B)
/// ... 0  0  0 [1] 1  0 ... (after 2 steps;    about to run state A)
/// ... 0  0 [0] 0  1  0 ... (after 3 steps;    about to run state B)
/// ... 0 [0] 1  0  1  0 ... (after 4 steps;    about to run state A)
/// ... 0  1 [1] 0  1  0 ... (after 5 steps;    about to run state B)
/// ... 0  1  1 [0] 1  0 ... (after 6 steps;    about to run state A)
/// ```
///
/// The CPU can confirm that the Turing machine is working by taking a
/// *diagnostic checksum* after a specific number of steps (given in the
/// blueprint). Once the specified number of steps have been executed, the
/// Turing machine should pause; once it does, count the number of times `1`
/// appears on the tape. In the above example, the *diagnostic checksum* is
/// *`3`*.
///
/// ```
/// # use advent_solutions::advent2017::day25::{ parse_input, part1 };
/// # let input = parse_input("\
/// # Begin in state A.
/// # Perform a diagnostic checksum after 6 steps.
/// #
/// # In state A:
/// #   If the current value is 0:
/// #     - Write the value 1.
/// #     - Move one slot to the right.
/// #     - Continue with state B.
/// #   If the current value is 1:
/// #     - Write the value 0.
/// #     - Move one slot to the left.
/// #     - Continue with state B.
/// #
/// # In state B:
/// #   If the current value is 0:
/// #     - Write the value 1.
/// #     - Move one slot to the left.
/// #     - Continue with state A.
/// #   If the current value is 1:
/// #     - Write the value 1.
/// #     - Move one slot to the right.
/// #     - Continue with state A.
/// # ");
/// #
/// assert_eq!(part1(&input), 3);
/// ```
///
/// Recreate the Turing machine and save the computer! *What is the
/// diagnostic checksum* it produces once it's working again?
pub fn part1(input: &Blueprint) -> usize {
    use std::collections::VecDeque;

    let mut tape = VecDeque::new();
    tape.push_back(false);

    let mut ip = 0;
    let mut state = input.begin;

    for _ in 0..input.checksum_after {
        if tape[ip] == false {
            let State { false_write, false_right, false_next, .. } = input.states[&state];

            tape[ip] = false_write;

            if false_right {
                ip += 1;

                if ip == tape.len() {
                    tape.push_back(false);
                }
            } else {
                if ip == 0 {
                    tape.push_front(false);
                } else {
                    ip -= 1;
                }
            };

            state = false_next;
        } else {
            let State { true_write, true_right, true_next, .. } = input.states[&state];

            tape[ip] = true_write;

            if true_right {
                ip += 1;

                if ip == tape.len() {
                    tape.push_back(false);
                }
            } else {
                if ip == 0 {
                    tape.push_front(false);
                } else {
                    ip -= 1;
                }
            };

            state = true_next;
        }
    }

    tape.into_iter().filter(|&x| x).count()
}

/// The Turing machine, and soon the entire computer, springs back to life.
/// A console glows dimly nearby, awaiting your command.
///
/// ```text
/// > reboot printer
/// Error: That command requires priority 50. You currently have priority 0.
/// You must deposit 50 stars to increase your priority to the required level.
/// ```
///
/// The console flickers for a moment, and then prints another message:
///
/// ```text
/// Star accepted.
/// You must deposit 49 stars to increase your priority to the required level.
/// ```
/// The *garbage collector* winks at you, then continues sweeping.
pub fn part2(_: &Blueprint) -> &str {
    ""
}

pub fn parse_input(input: &str) -> Blueprint {
    Blueprint::from_bytes(input.as_bytes())
        .to_full_result()
        .expect("Error parsing blueprint")
}

test_day!("25", 2846, "");
