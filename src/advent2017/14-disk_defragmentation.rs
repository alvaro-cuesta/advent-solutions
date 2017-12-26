//! # [Day 14: Disk Defragmentation](http://adventofcode.com/2017/day/14)
//!
//! Suddenly, a scheduled job activates the system's [disk defragmenter].
//! Were the situation different, you might [sit and watch it for a while],
//! but today, you just don't have that kind of time. It's soaking up
//! valuable system resources that are needed elsewhere, and so the only
//! option is to help it finish its task as soon as possible.
//!
//!   [disk defragmenter]: https://en.wikipedia.org/wiki/Defragmentation
//!   [sit and watch it for a while]: https://www.youtube.com/watch?v=kPv1gQ5Rs8A&t=37

use std::fmt;
use super::knot_hash;

/// The disk in question consists of a 128x128 grid; each square of the grid
/// is either *free* or *used*. On this disk, the state of the grid is
/// tracked by the bits in a sequence of [knot hashes].
///
///   [knot hashes]: ../day10/index.html
pub struct Grid(Vec<Vec<bool>>);

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter()
            .map(|line| {
                line.iter().map(|&x| write!(f, "{}", if x { '#' } else { '.' }))
                    .skip_while(|x| x.is_ok())
                    .next()
                    .unwrap_or(Ok(()))
                    .and_then(|_| write!(f, "\n"))
            })
            .skip_while(|x| x.is_ok())
            .next()
            .unwrap_or(Ok(()))
            .and_then(|_| write!(f, "\n"))
    }
}

/// A total of 128 knot hashes are calculated, each corresponding to a
/// single row in the grid; each hash contains 128 bits which correspond to
/// individual grid squares. Each bit of a hash indicates whether that
/// square is *free* (`0`) or *used* (`1`).
///
/// The hash inputs are a key string (your puzzle input), a dash, and a
/// number from `0` to `127` corresponding to the row. For example, if your
/// key string were `flqrgnkx`, then the first row would be given by the
/// bits of the knot hash of `flqrgnkx-0`, the second row from the bits of
/// the knot hash of `flqrgnkx-1`, and so on until the last row,
/// `flqrgnkx-127`.
pub fn make_hashes(input: &str) -> Vec<Vec<u8>> {
    (0..128)
        .map(|i| {
            let i_str = format!("{}-{}", input, i);
            knot_hash::hash_str(&i_str, 64)
        })
        .collect::<Vec<_>>()
}

/// The output of a knot hash is traditionally represented by 32 hexadecimal
/// digits; each of these digits correspond to 4 bits, for a total of
/// `4 * 32 = 128` bits. To convert to bits, turn each hexadecimal digit to
/// its equivalent binary value, high-bit first: `0` becomes `0000`, `1`
/// becomes `0001`, `e` becomes `1110`, `f` becomes `1111`, and so on; a
/// hash that begins with `a0c2017...` in hexadecimal would begin with
/// `10100000110000100000000101110000...` in binary.
///
/// Continuing this process, the *first 8 rows and columns* for key
/// `flqrgnkx` appear as follows, using `#` to denote used squares, and `.`
/// to denote free ones:
///
/// ```text
/// ##.#.#..-->
/// .#.#.#.#
/// ....#.#.
/// #.#.##.#
/// .##.#...
/// ##..#..#
/// .#...#..
/// ##.#.##.-->
/// |      |
/// V      V
/// ```
///
/// In this example, `8108` squares are used across the entire 128x128 grid.
///
/// ```
/// # use advent_solutions::advent2017::day14::{ make_hashes, part1 };
/// let hashes = make_hashes("flqrgnkx");
/// assert_eq!(part1(&hashes), 8108);
/// ```
///
/// Given your actual key string, *how many squares are used*?
pub fn part1<'a, I, J>(hashes: I) -> u32
    where I: IntoIterator<Item=J>,
          J: IntoIterator<Item=&'a u8>,
{
    hashes.into_iter()
        .map(|row| row.into_iter()
            .map(|x| x.count_ones())
            .sum::<u32>()
        )
        .sum::<u32>()
}

/// Now, <span title="This is exactly how it works in real life.">all the
/// defragmenter needs to know</span> is the number of *regions*. A region
/// is a group of *used* squares that are all *adjacent*, not including
/// diagonals. Every used square is in exactly one region: lone used squares
/// form their own isolated regions, while several adjacent squares all
/// count as a single region.
///
/// In the example above, the following nine regions are visible, each
/// marked with a distinct digit:
///
/// ```text
/// 11.2.3..-->
/// .1.2.3.4
/// ....5.6.
/// 7.8.55.9
/// .88.5...
/// 88..5..8
/// .8...8..
/// 88.8.88.-->
/// |      |
/// V      V
/// ```
///
/// Of particular interest is the region marked `8`; while it does not
/// appear contiguous in this small view, all of the squares marked `8` are
/// connected when considering the whole 128x128 grid. In total, in this
/// example, `1242` regions are present.
///
/// ```
/// # use advent_solutions::advent2017::day14::{ make_hashes, part2 };
/// let hashes = make_hashes("flqrgnkx");
/// assert_eq!(part2(&hashes), 1242);
/// ```
///
/// *How many regions* are present given your key string?
pub fn part2<'a, I, J>(hashes: I) -> usize
    where I: IntoIterator<Item=J>,
          J: IntoIterator<Item=&'a u8>,
{
    let grid = Grid(
        hashes.into_iter()
            .map(|row| row.into_iter()
                .flat_map(|x| ::iter::Bits::new(*x as usize, 8))
                .collect()
            )
            .collect()
    );

    use std::collections::HashSet;

    let mut ungrouped = grid.0.iter()
        .enumerate()
        .flat_map(|(y, row)| ::std::iter::repeat(y)
            .zip(0..row.len())
        )
        .filter(|&(y, x)| grid.0[y][x])
        .collect::<HashSet<(usize, usize)>>();

    let mut groups = Vec::new();

    while let Some(&(y, x)) = ungrouped.iter().next() {
        let mut group = Vec::new();

        let mut candidates = vec![(y, x)];

        while let Some((y, x)) = candidates.pop() {
            if ungrouped.contains(&(y, x)) {
                group.push((y, x));

                if y < 127 { candidates.push((y + 1, x)); }
                if y > 0   { candidates.push((y - 1, x)); }
                if x < 127 { candidates.push((y, x + 1)); }
                if x > 0   { candidates.push((y, x - 1)); }

                ungrouped.remove(&(y, x));
            }
        }

        groups.push(group);
    }

    groups.len()
}

pub fn main(download: &::Download) {
    let hashes = make_hashes(&download.single_input(2017, 14));

    println!("Part 1: {}", part1(&hashes));
    println!("Part 2: {}", part2(&hashes));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_input() {
        let mut input = include_str!("../../test_inputs/2017/14");
        input = &input[..input.len() - 1];

        let hashes = super::make_hashes(input);

        assert_eq!(super::part1(&hashes), 8222);
        assert_eq!(super::part2(&hashes), 1086);
    }
}
