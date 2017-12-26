//! # [Day 11: Hex Ed](http://adventofcode.com/2017/day/11)
//!
//! Crossing the bridge, you've barely reached the other side of the stream
//! when a program comes up to you, clearly in distress. "It's my child
//! process," she says, "he's gotten lost in an infinite grid!"
//!
//! Fortunately for her, you have plenty of experience with infinite grids.
//!
//! Unfortunately for you, it's a [hex grid].

use std::str::FromStr;
use std::cmp;
use std::ops::*;

/// The hexagons ("hexes") in <span title="Raindrops on roses and whiskers on
/// kittens.">this grid</span> are aligned such that adjacent hexes can be
/// found to the north, northeast, southeast, south, southwest, and northwest:
///
/// ```text
///       \ n  /
///     nw +--+ ne
///       /    \
///     -+      +-
///       \    /
///     sw +--+ se
///       / s  \
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum HexDirection { N, NE, SE, S, SW, NW }

use self::HexDirection::*;

impl<'a> FromStr for HexDirection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(N),
            "ne" => Ok(NE),
            "se" => Ok(SE),
            "s" => Ok(S),
            "sw" => Ok(SW),
            "nw" => Ok(NW),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct HexCoord(isize, isize);

impl HexCoord {
    fn distance(&self, other: &HexCoord) -> usize {
        let &HexCoord(aq, ar) = self;
        let &HexCoord(bq, br) = other;

        (
            (aq - bq).abs()
            + (aq + ar - bq - br).abs()
            + (ar - br).abs()
        ) as usize / 2
    }
}

impl Add<HexDirection> for HexCoord {
    type Output = HexCoord;

    fn add(self, direction: HexDirection) -> Self::Output {
        let HexCoord(q, r) = self;

        match direction {
            N => HexCoord(q, r - 1),
            NE => HexCoord(q + 1, r - 1),
            SE => HexCoord(q + 1, r),
            S => HexCoord(q, r + 1),
            SW => HexCoord(q - 1, r + 1),
            NW => HexCoord(q - 1, r),
        }
    }
}

/// You have the path the child process took. Starting where he started, you
/// need to determine the fewest number of steps required to reach him. (A
/// "step" means to move from the hex you are in to any adjacent hex.)
///
/// For example:
///
/// -   `ne,ne,ne` is `3` steps away.
///
/// ```
/// # use advent_solutions::advent2017::day11::solve;
/// assert_eq!(solve("ne,ne,ne").1, 3);
/// ```
///
/// -   `ne,ne,sw,sw` is `0` steps away (back where you started).
///
/// ```
/// # use advent_solutions::advent2017::day11::solve;
/// assert_eq!(solve("ne,ne,sw,sw").1, 0);
/// ```
///
/// -   `ne,ne,s,s` is `2` steps away (`se,se`).
///
/// ```
/// # use advent_solutions::advent2017::day11::solve;
/// assert_eq!(solve("ne,ne,s,s").1, 2);
/// ```
///
/// -   `se,sw,se,sw,sw` is `3` steps away (`s,s,sw`).
///
/// ```
/// # use advent_solutions::advent2017::day11::solve;
/// assert_eq!(solve("se,sw,se,sw,sw").1, 3);
/// ```
///
///   [hex grid]: https://en.wikipedia.org/wiki/Hexagonal_tiling
///
/// ## Part Two
///
/// *How many steps away* is the *furthest* he ever got from his starting
/// position?
pub fn solve(input: &str) -> (usize, usize) {
    let (max_distance, final_position) = input.split(',')
        .map(|dir| FromStr::from_str(dir).expect("Unknown direction"))
        .fold((0, HexCoord(0, 0)), |(furthest, coord), x| {
            let new_coord = coord + x;
            let distance = new_coord.distance(&HexCoord(0, 0));

            (cmp::max(distance, furthest), new_coord)
        });

    (max_distance, final_position.distance(&HexCoord(0, 0)))
}

pub fn main(download: &::Download) {
    let (max_distance, distance) = solve(&download.single_input(2017, 11));

    println!("Part 1: {}", distance);
    println!("Part 2: {}", max_distance);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_input() {
        let mut input = include_str!("../../test_inputs/2017/11");
        input = &input[..input.len() - 1];

        let (max_distance, distance) = super::solve(input);

        assert_eq!(max_distance, 1501);
        assert_eq!(distance, 759);
    }
}
