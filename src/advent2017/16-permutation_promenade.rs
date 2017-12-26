//! # [Day 16: Permutation Promenade](http://adventofcode.com/2017/day/16)
//!
//! You come upon a very unusual sight; a group of programs here appear to
//! be [dancing](https://www.youtube.com/watch?v=lyZQPjUT5B4&t=53).

use nom::anychar;
use ::parse::unsigned_number;

/// There are sixteen programs in total, named `a` through `p`. They start
/// by standing in a <span title="This is called a 'newline'.">line</span>:
/// `a` stands in position `0`, `b` stands in position `1`, and so on until
/// `p`, which stands in position `15`.
fn initial_programs() -> Vec<char> {
    vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ]
}

/// The programs' *dance* consists of a sequence of *dance moves*:
///
/// -   *Spin*, written `sX`, makes `X` programs move from the end to the
///     front, but maintain their order otherwise. (For example, `s3` on
///     `abcde` produces `cdeab`).
///
///     ```
///     # use advent_solutions::advent2017::day16::Move;
///     assert_eq!(
///         Move::Spin(3).apply(vec!['a', 'b', 'c', 'd', 'e']),
///         ['c', 'd', 'e', 'a', 'b']
///     )
///     ```
///
/// -   *Exchange*, written `xA/B`, makes the programs at positions `A` and
///     `B` swap places.
/// -   *Partner*, written `pA/B`, makes the programs named `A` and `B` swap
///     places.
///
/// For example, with only five programs standing in a line (`abcde`), they
/// could do the following dance:
///
/// ```
/// let starting_programs = vec!['a', 'b', 'c', 'd', 'e'];
/// ```
///
/// -   `s1`, a spin of size `1`: `eabcd`.
///
///     ```
///     # use advent_solutions::advent2017::day16::Move;
///     # let starting_programs = vec!['a', 'b', 'c', 'd', 'e'];
///     assert_eq!(
///         Move::Spin(1).apply(starting_programs),
///         ['e', 'a', 'b', 'c', 'd']
///     )
///     ```
///
/// -   `x3/4`, swapping the last two programs: `eabdc`.
///
///     ```
///     # use advent_solutions::advent2017::day16::Move;
///     # let after_s1 = vec!['e', 'a', 'b', 'c', 'd'];
///     assert_eq!(
///         Move::Exchange(3, 4).apply(after_s1),
///         ['e', 'a', 'b', 'd', 'c']
///     )
///     ```
///
/// -   `pe/b`, swapping programs `e` and `b`: `baedc`.
///
///     ```
///     # use advent_solutions::advent2017::day16::Move;
///     # let after_x34 = vec!['e', 'a', 'b', 'd', 'c'];
///     assert_eq!(
///         Move::Partner('e', 'b').apply(after_x34),
///         ['b', 'a', 'e', 'd', 'c']
///     )
///     ```
///
/// After finishing their dance, the programs end up in order `baedc`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

use self::Move::*;

impl Move {
    pub fn apply(&self, mut programs: Vec<char>) -> Vec<char> {
        match *self {
            Spin(length) => {
                let total = programs.len();
                let mut end = programs.split_off(total - length);
                end.extend(programs);
                end
            },
            Exchange(a, b) => {
                programs.swap(a, b);
                programs
            },
            Partner(a, b) => {
                let a_i = programs.iter().position(|&x| x == a).unwrap();
                let b_i = programs.iter().position(|&x| x == b).unwrap();
                programs.swap(a_i, b_i);
                programs
            },
        }
    }

    named!{ parse_spin (&[u8]) -> Move,
        do_parse!(
            char!('s') >>
            x: unsigned_number >>

            (Spin(x))
        )
    }

    named!{ parse_exchange (&[u8]) -> Move,
        do_parse!(
            char!('x') >>
            a: unsigned_number >>
            char!('/') >>
            b: unsigned_number >>

            (Exchange(a, b))
        )
    }

    named!{ parse_partner (&[u8]) -> Move,
        do_parse!(
            char!('p') >>
            a: anychar >>
            char!('/') >>
            b: anychar >>

            (Partner(a, b))
        )
    }

    named!{ pub list_from_bytes (&[u8]) -> Vec<Move>,
        separated_list!(
            char!(','),
            alt!( call!(Move::parse_spin)
                | call!(Move::parse_exchange)
                | call!(Move::parse_partner)
            )
        )
    }
}

fn dance(mut programs: Vec<char>, moves: &[Move]) -> Vec<char> {
    for m in moves {
        programs = m.apply(programs);
    }

    programs
}

/// You watch the dance for a while and record their dance moves (your
/// puzzle input). *In what order are the programs standing* after their
/// dance?
pub fn part1(moves: &[Move]) -> String {
    dance(initial_programs(), moves)
        .into_iter().collect()
}

fn find_cycle(moves: &[Move]) -> usize {
    let programs = initial_programs();
    let mut p = initial_programs();

    for i in 1.. {
        p = dance(p, moves);

        if programs == p {
            return i;
        }
    }

    unreachable!();
}

/// Now that you're starting to get a feel for the dance moves, you turn
/// your attention to *the dance as a whole*.
///
/// Keeping the positions they ended up in from their previous dance, the
/// programs perform it again and again: including the first dance, a total
/// of *one billion* (`1000000000`) times.
///
/// In the example above, their second dance would *begin* with the order
/// `baedc`, and use the same dance moves:
///
/// -   `s1`, a spin of size `1`: `cbaed`.
/// -   `x3/4`, swapping the last two programs: `cbade`.
/// -   `pe/b`, swapping programs `e` and `b`: `ceadb`.
///
/// *In what order are the programs standing* after their billion dances?
pub fn part2(moves: &[Move]) -> String {
    let dance_times = 1_000_000_000 % find_cycle(moves);
    println!("{}", dance_times);

    let mut programs = initial_programs();

    for _ in 0..dance_times {
        programs = dance(programs, moves);
    }

    programs.into_iter().collect()
}

pub fn main() {
    let input = ::download::input(2017, 16);

    let moves = Move::list_from_bytes(input.as_bytes())
            .to_full_result()
            .expect("Error parsing moves");

    println!("Part 1: {}", part1(&moves));
    println!("Part 2: {}", part2(&moves));
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_input() {
        let input = include_str!("../../test_inputs/2017/16");

        let moves = super::Move::list_from_bytes(input.as_bytes())
            .to_full_result()
            .expect("Error parsing moves");

        assert_eq!(super::part1(&moves), "nlciboghjmfdapek");
        assert_eq!(super::part2(&moves), "nlciboghmkedpfja");
    }
}
