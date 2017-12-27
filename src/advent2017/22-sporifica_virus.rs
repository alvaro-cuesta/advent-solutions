//! # [Day 22: Sporifica Virus](http://adventofcode.com/2017/day/22)
//!
//! Diagnostics indicate that the local *grid computing cluster* has been
//! contaminated with the *Sporifica Virus*. The grid computing cluster is a
//! seemingly-<span title="The infinite is possible at AdventOfCodeCom.">
//! infinite</span> two-dimensional grid of compute nodes. Each node is either
//! *clean* or *infected* by the virus.

use std::{ fmt, ops };
use std::convert::TryFrom;
use std::collections::VecDeque;

use ::Direction;
use ::Direction::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Node { Clean, Weakened, Infected, Flagged }

use self::Node::*;

impl TryFrom<char> for Node {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Clean),
            'W' => Ok(Weakened),
            '#' => Ok(Infected),
            'F' => Ok(Flagged),
            _ => Err("Invalid node character"),
        }
    }
}

impl Into<char> for Node {
    fn into(self) -> char {
        match self {
            Clean => '.',
            Weakened => 'W',
            Infected => '#',
            Flagged => 'F',
        }
    }
}

impl<'a> Into<char> for &'a Node {
    fn into(self) -> char {
        match *self {
            Clean => '.',
            Weakened => 'W',
            Infected => '#',
            Flagged => 'F',
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<char>::into(self))
    }
}

/// Diagnostics have also provided a *map of the node infection status*
/// (your puzzle input). *Clean* nodes are shown as `.`; *infected* nodes
/// are shown as `#`. This map only shows the center of the grid; there are
/// many more nodes beyond those shown, but none of them are currently
/// infected.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Memory {
    memory: VecDeque<VecDeque<Node>>,
    origin: (usize, usize),
}

impl Memory {
    fn parse(input: &str) -> Memory {
        let memory = input.lines()
            .map(|line| line.chars()
                .map(|c| Node::try_from(c).unwrap())
                .collect::<VecDeque<_>>()
            )
            .collect::<VecDeque<_>>();

        let height = memory.len();
        let width = memory[0].len();

        Memory {
            memory,
            origin: (height / 2, width / 2),
        }
    }

    fn width(&self) -> usize {
        self.memory[0].len()
    }

    fn height(&self) -> usize {
        self.memory.len()
    }
}

impl ops::Index<(isize, isize)> for Memory {
    type Output = Node;
    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        let ny = self.origin.0 as isize + y;
        let nx = self.origin.1 as isize + x;

        if ny >= self.height() as isize
            || ny < 0
            || nx >= self.width() as isize
            || nx < 0 {
            return &Clean;
        }

        &self.memory[ny as usize][nx as usize]
    }
}

impl ops::IndexMut<(isize, isize)> for Memory {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
        let ny = self.origin.0 as isize + y;
        let nx = self.origin.1 as isize + x;

        if ny >= self.height() as isize {
            let grow = ny - self.height() as isize + 1;

            for _ in 0..grow {
                let mut row = VecDeque::new();
                row.resize(self.width(), Clean);
                self.memory.push_back(row);
            }
        } else if ny < 0 {
            let grow = ny.abs();

            for _ in 0..grow {
                let mut row = VecDeque::new();
                row.resize(self.width(), Clean);
                self.memory.push_front(row);
            }

            self.origin.0 += grow as usize;
        }

        let ny = self.origin.0 as isize + y;

        if nx >= self.width() as isize {
            let grow = nx - self.width() as isize + 1;

            self.memory.iter_mut().for_each(|row| for _ in 0..grow {
                row.push_back(Clean)
            });
        } else if nx < 0 {
            let grow = nx.abs();

            self.memory.iter_mut().for_each(|row| for _ in 0..grow {
                row.push_front(Clean)
            });

            self.origin.1 += grow as usize;
        }

        let nx = self.origin.1 as isize + x;

        &mut self.memory[ny as usize][nx as usize]
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.memory.iter()
            .map(|line| {
                line.iter().map(|x| write!(f, "{}", x))
                    .skip_while(|x| x.is_ok())
                    .next()
                    .unwrap_or(Ok(()))
                    .and_then(|_| write!(f, "\n"))
            })
            .skip_while(|x| x.is_ok())
            .next()
            .unwrap_or(Ok(()))
            .and_then(|_| (0..self.width())
                .map(|i| write!(f, "{}", if i == self.origin.1 { "|" } else { " " }))
                .skip_while(|x| x.is_ok())
                .next()
                .unwrap_or(Ok(()))
                .and_then(|_| write!(f, "\n"))
            )
            .and_then(|_| write!(f, "\n"))
    }
}

/// To [prevent overloading] the nodes (which would render them useless to
/// the virus) or detection by system administrators, exactly one *virus
/// carrier* moves through the network, infecting or cleaning nodes as it
/// moves. The virus carrier is always located on a single node in the
/// network (the *current node*) and keeps track of the *direction* it is
/// facing.
///
/// The virus carrier begins in the middle of the map facing *up*.
///
///   [prevent overloading]: https://en.wikipedia.org/wiki/Morris_worm#The_mistake
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Carrier {
    position: (isize, isize),
    facing: Direction,
    infected: usize,
}

impl Carrier {
    fn new() -> Carrier {
        Carrier {
            position: (0, 0),
            facing: Up,
            infected: 0,
        }
    }

    /// To avoid detection, the virus carrie/// r works in bursts; in each burst, it
    /// *wakes up*, does some *work*, and goes back to *sleep*. The following
    /// steps are all executed *in order* one time each burst:
    ///
    /// -   If the *current node* is *infected*, it turns to its *right*.
    ///     Otherwise, it turns to its *left*. (Turning is done in-place; the
    ///     *current node* does not change.)
    /// -   If the *current node* is *clean*, it becomes *infected*. Otherwise,
    ///     it becomes *cleaned*. (This is done *after* the node is considered
    ///     for the purposes of changing direction.)
    /// -   The virus carrier [moves] *forward* one node in the direction it is
    ///     facing.
    ///
    ///   [moves]: https://www.youtube.com/watch?v=2vj37yeQQHg
    fn part1_with_bursts(&mut self, memory: &mut Memory) {
        match memory[self.position] {
            Clean => {
                self.facing = self.facing.ccw();
                memory[self.position] = Infected;
                self.infected += 1;
            },
            Infected => {
                self.facing = self.facing.cw();
                memory[self.position] = Clean;
            },
            _ => panic!("Step 1 should not have weakened or flagged nodes"),
        };

        self.position += self.facing;
    }

    /// As you go to remove the virus from the infected nodes, it *evolves* to
    /// resist your attempt.
    ///
    /// Now, before it infects a clean node, it will *weaken* it to disable your
    /// defenses. If it encounters an infected node, it will instead *flag* the
    /// node to be cleaned in the future. So:
    ///
    /// -   *Clean* nodes become *weakened*.
    /// -   *Weakened* nodes become *infected*.
    /// -   *Infected* nodes become *flagged*.
    /// -   *Flagged* nodes become *clean*.
    ///
    /// Every node is always in exactly one of the above states.
    ///
    /// The virus carrier still functions in a similar way, but now uses the
    /// following logic during its bursts of action:
    ///
    /// -   Decide which way to turn based on the *current node*:
    ///     -   If it is *clean*, it turns *left*.
    ///     -   If it is *weakened*, it does *not* turn, and will continue
    ///         moving in the same direction.
    ///     -   If it is *infected*, it turns *right*.
    ///     -   If it is *flagged*, it *reverses* direction, and will go back
    ///         the way it came.
    /// -   Modify the state of the *current node*, as described above.
    /// -   The virus carrier moves *forward* one node in the direction it is
    ///     facing.
    pub fn part2_with_bursts(&mut self, memory: &mut Memory) {
        match memory[self.position] {
            Clean => {
                self.facing = self.facing.ccw();
                memory[self.position] = Weakened;
            },
            Weakened => {
                memory[self.position] = Infected;
                self.infected += 1;
            },
            Infected => {
                self.facing = self.facing.cw();
                memory[self.position] = Flagged;
            },
            Flagged => {
                self.facing = self.facing.reverse();
                memory[self.position] = Clean;
            },
        };

        self.position += self.facing;
    }
}


/// For example, suppose you are given a map like this:
///
/// ```text
/// ..#
/// #..
/// ...
/// ```
///
/// Then, the middle of the infinite grid looks like this, with the virus
/// carrier's position marked with `[ ]`:
///
/// ```text
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . # . . .
/// . . . #[.]. . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// ```
///
/// The virus carrier is on a *clean* node, so it turns *left*, *infects*
/// the node, and moves left:
///
/// ```text
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . # . . .
/// . . .[#]# . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// ```
///
/// The virus carrier is on an *infected* node, so it turns *right*,
/// *cleans* the node, and moves up:
///
/// ```text
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . .[.]. # . . .
/// . . . . # . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// ```
///
/// Four times in a row, the virus carrier finds a *clean*, *infects* it,
/// turns *left*, and moves forward, ending in the same place and still
/// facing up:
///
/// ```text
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . #[#]. # . . .
/// . . # # # . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// ```
///
/// Now on the same node as before, it sees an infection, which causes it to
/// turn *right*, *clean* the node, and move forward:
///
/// ```text
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . # .[.]# . . .
/// . . # # # . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// ```
///
/// After the above actions, a total of `7` bursts of activity had taken
/// place. Of them, `5` bursts of activity caused an infection.
///
/// After a total of `70`, the grid looks like this, with the virus carrier
/// facing up:
///
/// ```text
/// . . . . . # # . .
/// . . . . # . . # .
/// . . . # . . . . #
/// . . # . #[.]. . #
/// . . # . # . . # .
/// . . . . . # # . .
/// . . . . . . . . .
/// . . . . . . . . .
/// ```
///
/// By this time, `41` bursts of activity caused an infection (though most
/// of those nodes have since been cleaned).
///
/// ```
/// # use advent_solutions::advent2017::day22::part1_with_bursts;
/// # let input = "..#
/// # #..
/// # ...
/// # ";
/// assert_eq!(part1_with_bursts(input, 70), 41);
/// ```
///
/// After a total of `10000` bursts of activity, `5587` bursts will have
/// caused an infection.
///
/// ```
/// # use advent_solutions::advent2017::day22::part1_with_bursts;
/// # let input = "..#
/// # #..
/// # ...
/// # ";
/// assert_eq!(part1_with_bursts(input, 10000), 5587);
/// ```
///
/// Given your actual map, after `10000` bursts of activity, *how many
/// bursts cause a node to become infected*? (Do not count nodes that begin
/// infected.)
pub fn part1_with_bursts(input: &str, bursts: usize) -> usize {
    let mut memory = Memory::parse(input);
    let mut carrier = Carrier::new();

    for _ in 0..bursts {
        carrier.part1_with_bursts(&mut memory);
    }

    carrier.infected
}

pub fn part1(input: &str) -> usize {
    part1_with_bursts(input, 10000)
}

/// Start with the same map (still using `.` for *clean* and `#` for
/// infected) and still with the virus carrier starting in the middle and
/// facing *up*.
///
/// Using the same initial state as the previous example, and drawing
/// *weakened* as `W` and *flagged* as `F`, the middle of the infinite grid
/// looks like this, with the virus carrier's position again marked with
/// `[ ]`:
///
/// ```text
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . # . . .
/// . . . #[.]. . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// ```
///
/// This is the same as before, since no initial nodes are *weakened* or
/// *flagged*. The virus carrier is on a clean node, so it still turns left,
/// instead *weakens* the node, and moves left:
///
/// ```text
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . # . . .
/// . . .[#]W . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// ```
///
/// The virus carrier is on an infected node, so it still turns right,
/// instead *flags* the node, and moves up:
///
/// ```text
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . .[.]. # . . .
/// . . . F W . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// ```
///
/// This process repeats three more times, ending on the previously-flagged
/// node and facing right:
///
/// ```text
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . W W . # . . .
/// . . W[F]W . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// ```
///
/// Finding a flagged node, it reverses direction and *cleans* the node:
///
/// ```text
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . W W . # . . .
/// . .[W]. W . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// ```
///
/// The *weakened* node becomes infected, and it continues in the same
/// direction:
///
/// ```text
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . W W . # . . .
/// .[.]# . W . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// . . . . . . . . .
/// ```
///
/// Of the first `100` bursts, `26` will result in *infection*.
/// Unfortunately, another feature of this evolved virus is *speed*; of the
/// first `10000000` bursts, `2511944` will result in *infection*.
///
/// ```
/// # use advent_solutions::advent2017::day22::part2_with_bursts;
/// let input = "..#
/// #..
/// ...
/// ";
///
/// assert_eq!(part2_with_bursts(input, 100), 26);
/// assert_eq!(part2_with_bursts(input, 10000000), 2511944);
/// ```
///
/// Given your actual map, after `10000000` bursts of activity, *how many
/// bursts cause a node to become infected*? (Do not count nodes that begin
/// infected.)
pub fn part2_with_bursts(input: &str, bursts: usize) -> usize {
    let mut memory = Memory::parse(&input);
    let mut carrier = Carrier::new();

    for _ in 0..bursts {
        carrier.part2_with_bursts(&mut memory);
    }

    carrier.infected
}

pub fn part2(input: &str) -> usize {
    part2_with_bursts(input, 10000000)
}

pub fn parse_input(input: &str) -> &str {
    input
}

test_day!("22", 5259, 2511722);
