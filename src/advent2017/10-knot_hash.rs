//! # [Day 10: Knot Hash](http://adventofcode.com/2017/day/10)
//!
//! You come across some programs that are trying to implement a software
//! emulation of a hash based on knot-tying. The hash these programs are
//! implementing isn't very strong, but you decide to help them anyway. You
//! make a mental note to remind the Elves later not to <span
//! title="NEW CRYPTOSYSTEM WHO DIS">invent their own cryptographic
//! functions</span>.
//!
//! This hash function simulates tying a knot in a circle of string with 256
//! marks on it. Based on the input to be hashed, the function repeatedly
//! selects a span of string, brings the ends together, and gives the span a
//! half-twist to reverse the order of the marks within it. After doing this
//! many times, the order of the marks is used to build the resulting hash.
//!
//! ```text
//!       4--5   pinch   4  5           4   1
//!      /    \  5,0,1  / \/ \  twist  / \ / \
//!     3      0  -->  3      0  -->  3   X   0
//!      \    /         \ /\ /         \ / \ /
//!       2--1           2  1           2   5
//! ```
//!
//! See [knot_hash](../knot_hash/index.html)

use super::knot_hash;

/// However, you should instead use the standard list size of `256` (with
/// values `0` to `255`) and the sequence of *lengths* in your puzzle input.
/// Once this process is complete, *what is the result of multiplying the
/// first two numbers in the list*?
pub fn part1(input: &str) -> u16 {
    let lengths = input
        .split(',')
        .map(|l| l.parse::<u8>().expect("Unexpected non-u8 length"))
        .collect::<Vec<_>>();

    let hash = knot_hash::hash_lengths(255, &lengths, 1);

    (hash[0] as u16) * (hash[1] as u16)
}

/// Finally, the standard way to represent a Knot Hash is as a single
/// [hexadecimal] string; the final output is the dense hash in hexadecimal
/// notation. Because each number in your dense hash will be between `0` and
/// `255` (inclusive), always represent each number as two hexadecimal
/// digits (including a leading zero as necessary). So, if your first three
/// numbers are `64, 7, 255`, they correspond to the hexadecimal numbers
/// `40, 07, ff`, and so the first six characters of the hash would be
/// `4007ff`. Because every Knot Hash is sixteen such numbers, the
/// hexadecimal representation is always `32` hexadecimal digits (`0`-`f`)
/// long.
///
/// Here are some example hashes:
///
/// -   The empty string becomes `a2582a3a0e66e6e86e3812dcb672a272`.
///
/// ```
/// # use advent_solutions::advent2017::day10::part2;
/// assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
/// ```
///
/// -   `AoC 2017` becomes `33efeb34ea91902bb2f59c9920caa6cd`.
///
/// ```
/// # use advent_solutions::advent2017::day10::part2;
/// assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
/// ```
///
/// -   `1,2,3` becomes `3efbe78a8d82f29979031a4aa0b16a9d`.
///
/// ```
/// # use advent_solutions::advent2017::day10::part2;
/// assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
/// ```
///
/// -   `1,2,4` becomes `63960835bcdc130f0b66d7ff4f6a5a8e`.
///
/// ```
/// # use advent_solutions::advent2017::day10::part2;
/// assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
/// ```
///
/// Treating your puzzle input as a string of ASCII characters, *what is the
/// Knot Hash of your puzzle input?* Ignore any leading or trailing
/// whitespace you might encounter.
///
///   [hexadecimal]: https://en.wikipedia.org/wiki/Hexadecimal
pub fn part2(input: &str) -> String {
    knot_hash::hash_str(input, 64).iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>()
}

pub fn main() {
    let input = ::download::single_input(2017, 10);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
