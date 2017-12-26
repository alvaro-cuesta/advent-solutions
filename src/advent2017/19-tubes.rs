//! # [Day 19: A Series of Tubes](http://adventofcode.com/2017/day/19)
//!
//! Somehow, a network packet got <span title="I know how fast it's going, but I
//! don't know where it is.">lost</span> and ended up here. It's trying to follow
//! a routing diagram (your puzzle input), but it's confused about where to go.

use ::Direction;
use ::Direction::*;

/// Its starting point is just off the top of the diagram. Lines (drawn with
/// `|`, `-`, and `+`) show the path it needs to take, starting by going
/// down onto the only line connected to the top of the diagram. It needs to
/// follow this path until it reaches the end (located somewhere within the
/// diagram) and stop there.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Cell {
    Road,
    Empty,
    Letter(char),
}

use self::Cell::*;

type Grid = Vec<Vec<Cell>>;

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '-' | '|' | '+' => Road,
            ' ' => Empty,
            c => Letter(c),
        }
    }
}

/// Sometimes, the lines cross over each other; in these cases, it needs to
/// continue going the same direction, and only turn left or right when
/// there's no other option. In addition, someone has left *letters* on the
/// line; these also don't change its direction, but it can use them to keep
/// track of where it's been. For example:
///
/// ```text
///      |
///      |  +--+
///      A  |  C
///  F---|----E|--+
///      |  |  |  D
///      +B-+  +--+
/// ```
///
/// Given this diagram, the packet needs to take the following path:
///
/// -   Starting at the only line touching the top of the diagram, it must
///     go down, pass through `A`, and continue onward to the first `+`.
/// -   Travel right, up, and right, passing through `B` in the process.
/// -   Continue down (collecting `C`), right, and up (collecting `D`).
/// -   Finally, go all the way left through `E` and stopping at `F`.
///
/// Following the path to the end, the letters it sees on its path are
/// `ABCDEF`.
///
/// ```
/// # use advent_solutions::advent2017::day19::solve;
/// # let input = [
/// #     "     |          \n",
/// #     "     |  +--+    \n",
/// #     "     A  |  C    \n",
/// #     " F---|----E|--+ \n",
/// #     "     |  |  |  D \n",
/// #     "     +B-+  +--+ \n",
/// # ]
/// # .iter()
/// # .map(|x| *x)
/// # .collect::<String>();
/// let (collected, _) = solve(&input);
/// assert_eq!(collected, "ABCDEF");
/// ```
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Packet {
    position: (usize, usize),
    direction: Direction,
    collected: Vec<char>,
}

impl Packet {
    fn new(x: usize) -> Packet {
        Packet {
            position: (x, 0),
            direction: Down,
            collected: Vec::new(),
        }
    }

    fn step(&mut self, grid: &Grid) -> bool {
        if let Letter(c) = grid[self.position.1][self.position.0] {
            self.collected.push(c);
        }

        let n = self.neighbors(grid);

        if n.iter().any(|&x| x == self.direction) {
            self.position += self.direction;
        } else if n.iter().any(|&x| x == self.direction.cw()) {
            self.direction = self.direction.cw();
            self.position += self.direction;
        } else if n.iter().any(|&x| x == self.direction.ccw()) {
            self.direction = self.direction.ccw();
            self.position += self.direction;
        } else {
            return false;
        }

        true
    }

    fn neighbors(&self, grid: &Grid) -> Vec<Direction> {
        let w = grid[0].len();
        let h = grid.len();

        [Up, Down, Left, Right].iter()
            .map(|&dir| (dir, self.position + dir))
            .filter(|&(_, (x, y))|
                x > 0 && y > 0
                && x < w && y < h
                && grid[y as usize][x as usize] != Empty
            )
            .map(|(dir, _)| dir)
            .collect()
    }
}


/// The little packet looks up at you, hoping you can help it find the way.
/// *What letters will it see* (in the order it would see them) if it
/// follows the path? (The routing diagram is very wide; make sure you view
/// it without line wrapping.)
///
/// ## Part Two
///
/// The packet is curious how many steps it needs to go.
///
/// For example, using the same routing diagram from the example above...
///
/// ```text
///      |
///      |  +--+
///      A  |  C
///  F---|----E|--+
///      |  |  |  D
///      +B-+  +--+
/// ```
///
/// ...the packet would go:
///
/// -   `6` steps down (including the first line at the top of the diagram).
/// -   `3` steps right.
/// -   `4` steps up.
/// -   `3` steps right.
/// -   `4` steps down.
/// -   `3` steps right.
/// -   `2` steps up.
/// -   `13` steps left (including the `F` it stops on).
///
/// This would result in a total of `38` steps.
///
/// ```
/// # use advent_solutions::advent2017::day19::solve;
/// # let input = [
/// #     "     |          \n",
/// #     "     |  +--+    \n",
/// #     "     A  |  C    \n",
/// #     " F---|----E|--+ \n",
/// #     "     |  |  |  D \n",
/// #     "     +B-+  +--+ \n",
/// # ]
/// # .iter()
/// # .map(|x| *x)
/// # .collect::<String>();
/// let (_, steps) = solve(&input);
/// assert_eq!(steps, 38);
/// ```
///
/// *How many steps* does the packet need to go?
pub fn solve(input: &str) -> (String, usize) {
    let grid: Grid = input
        .lines()
        .map(|line| line.chars().map(Into::<Cell>::into).collect())
        .collect();

    let x = grid[0].iter().position(|&c| c == Road)
        .expect("Could not find starting point");

    let mut packet = Packet::new(x);
    let mut steps = 1;

    while packet.step(&grid) {
        steps += 1;
    }

    let collected = packet.collected.into_iter().collect::<String>();

    (collected, steps)
}

pub fn main() {
    let (collected, steps) = solve(&::download::input(2017, 19));

    println!("Part 1: {}", collected);
    println!("Part 2: {}", steps);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_input() {
        let input = include_str!("../../test_inputs/2017/19");

        let (collected, steps) = super::solve(input);

        assert_eq!(collected, "LXWCKGRAOY");
        assert_eq!(steps, 17302);
    }
}
