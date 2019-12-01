//! Solutions for [Advent of Code].
//!
//!  [Advent of Code]: http://adventofcode.com/about

extern crate itertools;
extern crate reqwest;
#[macro_use]
extern crate nom;

mod download;
pub use download::Downloader;

#[macro_use]
pub mod parse;

pub mod iter;

mod direction;
pub use direction::Direction;

macro_rules! test_day {
    ($day:expr, $part1:expr, $part2:expr) => {
        #[cfg(test)]
        mod tests {
            #[test]
            fn parse() {
                super::parse_input(include_str!(concat!("../../test_inputs/2017/", $day)));
            }

            #[test]
            fn part1() {
                let input =
                    super::parse_input(include_str!(concat!("../../test_inputs/2017/", $day)));

                assert_eq!(super::part1(&input), $part1);
            }

            #[test]
            fn part2() {
                let input =
                    super::parse_input(include_str!(concat!("../../test_inputs/2017/", $day)));

                assert_eq!(super::part2(&input), $part2);
            }
        }
    };
}

macro_rules! test_day_both {
    ($day:expr, $part1:expr, $part2:expr) => {
        #[cfg(test)]
        mod tests {
            #[test]
            fn parse() {
                super::parse_input(include_str!(concat!("../../test_inputs/2017/", $day)));
            }

            #[test]
            fn solve() {
                let input =
                    super::parse_input(include_str!(concat!("../../test_inputs/2017/", $day)));
                let (part1, part2) = super::solve(&input);

                assert_eq!(part1, $part1);
                assert_eq!(part2, $part2);
            }
        }
    };
}

pub mod advent2017;
