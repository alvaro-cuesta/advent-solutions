//! # [Day 3: Spiral Memory](http://adventofcode.com/2017/day/3)
//!
//! You come across an experimental new kind of memory stored on an <span
//! title="Good thing we have all these infinite two-dimensional grids lying
//! around!">infinite two-dimensional grid</span>.
//!
//! Each square on the grid is allocated in a spiral pattern starting at a
//! location marked `1` and then counting up while spiraling outward. For
//! example, the first few squares are allocated like this:
//!
//! ```text
//! 17  16  15  14  13
//! 18   5   4   3  12
//! 19   6   1   2  11
//! 20   7   8   9  10
//! 21  22  23---> ...
//! ```

use ::itertools::Itertools;
use ::Direction;
use ::Direction::*;

static DIRECTIONS: [Direction; 4] = [Right, Up, Left, Down];

static NEIGHBORS: [(isize, isize); 8] = [
    (-1, 1),  (0, 1),  (1, 1),
    (-1, 0),           (1, 0),
    (-1, -1), (0, -1), (1, -1),
];

fn spiral() -> impl Iterator<Item=&'static ::Direction> {
    (1..).interleave(1..)
    .zip(DIRECTIONS.iter().cycle())
    .flat_map(|(len, dir)| ::std::iter::repeat(dir).take(len))
}

/// While this is very space-efficient (no squares are skipped), requested
/// data must be carried back to square `1` (the location of the only access
/// port for this memory system) by programs that can only move up, down,
/// left, or right. They always take the shortest path: the [Manhattan
/// Distance] between the location of the data and square `1`.
///
/// For example:
///
/// -   Data from square `1` is carried `0` steps, since it's at the access
///     port.
///
///     ```
///     # use advent_solutions::advent2017::day03::part1;
///     assert_eq!(part1(1), 0);
///     ```
///
/// -   Data from square `12` is carried `3` steps, such as: down, left,
///     left.
///
///     ```
///     # use advent_solutions::advent2017::day03::part1;
///     assert_eq!(part1(12), 3);
///     ```
///
/// -   Data from square `23` is carried only `2` steps: up twice.
///
///     ```
///     # use advent_solutions::advent2017::day03::part1;
///     assert_eq!(part1(23), 2);
///     ```
///
/// -   Data from square `1024` must be carried `31` steps.
///
///     ```
///     # use advent_solutions::advent2017::day03::part1;
///     assert_eq!(part1(1024), 31);
///     ```
///
/// *How many steps* are required to carry the data from the square
/// identified in your puzzle input all the way to the access port?
///
///   [Manhattan Distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
pub fn part1(index: usize) -> usize {
    let (x, y) = spiral()
        .take(index - 1)
        .fold((0isize, 0isize), |pos, facing| pos + facing);

    (x.abs() + y.abs()) as usize
}

/// As a stress test on the system, the programs here clear the grid and
/// then store the value `1` in square `1`. Then, in the same allocation
/// order as shown above, they store the sum of the values in all adjacent
/// squares, including diagonals.
///
/// So, the first few squares' values are chosen as follows:
///
/// -   Square `1` starts with the value `1`.
/// -   Square `2` has only one adjacent filled square (with value `1`), so
///     it also stores `1`.
/// -   Square `3` has both of the above squares as neighbors and stores the
///     sum of their values, `2`.
/// -   Square `4` has all three of the aforementioned squares as neighbors
///     and stores the sum of their values, `4`.
/// -   Square `5` only has the first and fourth squares as neighbors, so it
///     gets the value `5`.
///
/// Once a square is written, its value does not change. Therefore, the
/// first few squares would receive the following values:
///
/// ```text
/// 147  142  133  122   59
/// 304    5    4    2   57
/// 330   10    1    1   54
/// 351   11   23   25   26
/// 362  747  806--->   ...
/// ```
///
/// ```
/// # use advent_solutions::advent2017::day03::stress_test;
/// let solution = [
///     1usize, 1, 2, 4, 5, 10, 11, 23,
///     25, 26, 54, 57, 59, 122, 133, 142,
///     147, 304, 330, 351, 362, 747, 806,
/// ];
///
/// assert!(stress_test().take(23).eq(solution.iter().cloned()));
/// ```
pub fn stress_test() -> impl Iterator<Item=usize> {
    use std::collections::HashMap;

    let mut cache = HashMap::new();
    cache.insert((0, 0), 1);

    ::std::iter::once(1)
        .chain(spiral()
            .scan(((0, 0), cache), |state, &facing| {
                (state.0) += facing;

                let val = ::std::cmp::max(
                    1,
                    NEIGHBORS.iter()
                        .map(|&(x, y)| state.1
                            .get(&((state.0).0 + x, (state.0).1 + y))
                            .unwrap_or(&0)
                        )
                        .sum::<usize>(),
                );

                state.1.insert(state.0, val);

                Some(val)
            })
        )
}

/// What is the *first value written* that is *larger* than your puzzle
/// input?
pub fn part2(index: usize) -> usize {
    stress_test()
        .skip_while(|&x| x < index)
        .next()
        .expect("Found no solution for step 2")
}

pub fn main(download: &::Download) {
    let index = download.single_input(2017, 3)
        .parse::<usize>()
        .expect("Unexpected non-integer");

    println!("Part 1: {}", part1(index));
    println!("Part 2: {}", part2(index));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_input() {
        let mut input = include_str!("../../test_inputs/2017/03");
        input = &input[..input.len() - 1];

        let val = input
            .parse::<usize>()
            .expect("Unexpected non-integer");

        assert_eq!(super::part1(val), 430);
        assert_eq!(super::part2(val), 312453);
    }
}
