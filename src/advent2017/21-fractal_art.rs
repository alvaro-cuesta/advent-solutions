//! # [Day 21: Fractal Art](http://adventofcode.com/2017/day/21)
//!
//! You find a program trying to generate some art. It uses a strange
//! process that involves <span title="This technique is also often used on
//! TV.">repeatedly enhancing</span> the detail of an image through a set of
//! rules.

use std::fmt;
use std::collections::HashMap;

/// The image consists of a two-dimensional square grid of pixels that are
/// either on (`#`) or off (`.`).
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Grid(Vec<Vec<bool>>);

impl Grid {
    fn region(&self, x: usize, y: usize, w: usize, h: usize) -> Grid {
        Grid(
            (0..h)
            .map(|yy| (0..w)
                .map(|xx| self.0[y + yy][x + xx])
                .collect()
            )
            .collect()
        )
    }

    fn split(&self) -> Vec<Vec<Grid>> {
        let size = self.0.len();

        if size % 2 == 0 {
            let tiles = size / 2;

            (0..tiles)
            .map(|y| (0..tiles)
                .map(|x| self.region(x * 2, y * 2, 2, 2))
                .collect()
            )
            .collect()
        } else {
            let tiles = size / 3;

            (0..tiles)
            .map(|y| (0..tiles)
                .map(|x| self.region(x * 3, y * 3, 3, 3))
                .collect()
            )
            .collect()
        }
    }

    /// When searching for a rule to use, rotate and flip the pattern as
    /// necessary. For example, all of the following patterns match the same
    /// rule:
    ///
    /// ```text
    /// .#.   .#.   #..   ###
    /// ..#   #..   #.#   ..#
    /// ###   ###   ##.   .#.
    /// ```
    fn flips_and_rotations(&self) -> Vec<Grid> {
        let mut result = vec![self.clone()];

        let size = self.0.len();

        let mut swap = self.clone();
        let mut flip_y = self.clone();
        let mut flip_y_swap = self.clone();
        let mut flip_x = self.clone();
        let mut flip_x_swap = self.clone();
        let mut flip_both = self.clone();
        let mut flip_both_swap = self.clone();

        for y in 0..size {
            for x in 0..size {
                swap.0[y][x] = self.0[x][y];
                flip_y.0[y][x] = self.0[size - y - 1][x];
                flip_y_swap.0[y][x] = self.0[x][size - y - 1];
                flip_x.0[y][x] = self.0[y][size - x - 1];
                flip_x_swap.0[y][x] = self.0[size - x - 1][y];
                flip_both.0[y][x] = self.0[size - y - 1][size - x - 1];
                flip_both_swap.0[y][x] = self.0[size - x - 1][size - y - 1];
            }
        }

        result.push(swap);
        result.push(flip_y);
        result.push(flip_y_swap);
        result.push(flip_x);
        result.push(flip_x_swap);
        result.push(flip_both);
        result.push(flip_both_swap);

        result
    }

    fn merge(tiles: Vec<Vec<Grid>>) -> Grid {
        let num_tiles = tiles.len();
        let size = tiles[0][0].0.len();

        let mut result = vec![];

        for y_tile in 0..num_tiles {
            for y in 0..size {
                let mut row = vec![];

                for x_tile in 0..num_tiles {
                    for x in 0..size {
                        row.push(tiles[y_tile][x_tile].0[y][x]);
                    }
                }

                result.push(row);
            }
        }

        Grid(result)
    }

    named!{ cell_from_bytes (&[u8]) -> Vec<bool>,
        many1!(
            map!(
                alt!(char!('#') | char!('.')),
                |x| x == '#'
            )
        )
    }

    named!{ pub from_bytes (&[u8]) -> Grid,
        map!(
            separated_list!(char!('/'), Grid::cell_from_bytes),
            |x| Grid(x)
        )
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.0.len();

        for y in 0..size {
            for x in 0..size {
                write!(f, "{}", if self.0[y][x] { '#' } else { '.' })?
            }

            write!(f, "\n")?
        }

        Ok(())
    }
}

/// The artist's book of enhancement rules is nearby (your puzzle input);
/// however, it seems to be missing rules. The artist explains that
/// sometimes, one must *rotate* or *flip* the input pattern to find a
/// match. (Never rotate or flip the output pattern, though.) Each pattern
/// is written concisely: rows are listed as single units, ordered top-down,
/// and separated by slashes. For example, the following rules correspond to
/// the adjacent patterns:
///
/// ```text
/// ../.#  =  ..
///           .#
///
///                 .#.
/// .#./..#/###  =  ..#
///                 ###
///
///                         #..#
/// #..#/..../#..#/.##.  =  ....
///                         #..#
///                         .##.
/// ```
type Rule = (Grid, Grid);

named!{ parse_rule (&[u8]) -> Rule,
    do_parse!(
        l: call!(Grid::from_bytes) >>
        tag!(" => ") >>
        r: call!(Grid::from_bytes) >>

        (l, r)
    )
}

named!{ parse_rules (&[u8]) -> Vec<Rule>,
    lines!(parse_rule)
}

/// The program always begins with this pattern:
///
/// ```text
/// .#.
/// ..#
/// ###
/// ```
///
/// Because the pattern is both `3` pixels wide and `3` pixels tall, it is
/// said to have a *size* of `3`.
///
/// Then, the program repeats the following process:
///
/// -   If the size is evenly divisible by `2`, break the pixels up into
///     `2x2` squares, and convert each `2x2` square into a `3x3` square by
///     following the corresponding *enhancement rule*.
/// -   Otherwise, the size is evenly divisible by `3`; break the pixels up
///     into `3x3` squares, and convert each `3x3` square into a `4x4`
///     square by following the corresponding *enhancement rule*.
///
/// Because each square of pixels is replaced by a larger one, the image
/// gains pixels and so its *size* increases.
///
/// Suppose the book contained the following two rules:
///
/// ../.# => ##./#../...
/// .#./..#/### => #..#/..../..../#..#
///
/// As before, the program begins with this pattern:
///
/// ```text
/// .#.
/// ..#
/// ###
/// ```
///
/// The size of the grid (`3`) is not divisible by `2`, but it is divisible
/// by `3`. It divides evenly into a single square; the square matches the
/// second rule, which produces:
///
/// ```text
/// #..#
/// ....
/// ....
/// #..#
/// ```
///
/// The size of this enhanced grid (`4`) is evenly divisible by `2`, so that
/// rule is used. It divides evenly into four squares:
///
/// ```text
/// #.|.#
/// ..|..
/// --+--
/// ..|..
/// #.|.#
/// ```
///
/// Each of these squares matches the same rule (`../.# => ##./#../...`),
/// three of which require some flipping and rotation to line up with the
/// rule. The output for the rule is the same in all four cases:
///
/// ```text
/// ##.|##.
/// #..|#..
/// ...|...
/// ---+---
/// ##.|##.
/// #..|#..
/// ...|...
/// ```
///
/// Finally, the squares are joined into a new grid:
///
/// ```text
/// ##.##.
/// #..#..
/// ......
/// ##.##.
/// #..#..
/// ......
/// ```
///
/// Thus, after `2` iterations, the grid contains `12` pixels that are *on*.
///
/// ```
/// # use advent_solutions::advent2017::day21::{ parse_input, Grid, solve };
/// let input = parse_input("\
/// ../.# => ##./#../...
/// .#./..#/### => #..#/..../..../#..#
/// ");
///
/// assert_eq!(solve(&input, 2), 12);
/// ```
pub fn solve(input: &HashMap<Grid, Grid>, iterations: usize) -> usize {
    let mut grid = Grid(
        vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ]
    );

    for _ in 0..iterations {
        let mut split = grid.split();

        for y in &mut split {
            for x in y {
                if input.contains_key(x) {
                    *x = input[x].clone()
                }
            }
        }

        grid = Grid::merge(split);
    }

    grid.0.into_iter()
        .map(|row| row.into_iter().filter(|&x| x).count())
        .sum::<usize>()
}

/// *How many pixels stay on* after `5` iterations?
pub fn part1(input: &HashMap<Grid, Grid>) -> usize {
    solve(input, 5)
}

/// *How many pixels stay on* after `18` iterations?
pub fn part2(input: &HashMap<Grid, Grid>) -> usize {
    solve(input, 18)
}

pub fn parse_input(input: &str) -> HashMap<Grid, Grid> {
    parse_rules(input.as_bytes())
        .to_full_result()
        .expect("Error parsing rules")
        .into_iter()
        .flat_map(|(k, v)| k.flips_and_rotations().into_iter()
            .map(|rot| (rot, v.clone()))
            .collect::<Vec<_>>()
        )
        .collect()
}

test_day!("21", 150, 2606275);
