#![feature(try_from)]

extern crate advent;
extern crate itertools;

use std::fmt;
use std::convert::TryFrom;
use std::collections::VecDeque;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Node { Clean, Weakened, Infected, Flagged }

impl TryFrom<char> for Node {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Node::*;

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
        use Node::*;

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
        use Node::*;

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

#[derive(Clone, PartialEq, Eq, Debug)]
struct Memory {
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

impl std::ops::Index<(isize, isize)> for Memory {
    type Output = Node;
    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        let ny = self.origin.0 as isize + y;
        let nx = self.origin.1 as isize + x;

        if ny >= self.height() as isize
            || ny < 0
            || nx >= self.width() as isize
            || nx < 0 {
            return &Node::Clean;
        }

        &self.memory[ny as usize][nx as usize]
    }
}

impl std::ops::IndexMut<(isize, isize)> for Memory {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
        let ny = self.origin.0 as isize + y;
        let nx = self.origin.1 as isize + x;

        if ny >= self.height() as isize {
            let grow = ny - self.height() as isize + 1;

            for _ in 0..grow {
                let mut row = VecDeque::new();
                row.resize(self.width(), Node::Clean);
                self.memory.push_back(row);
            }
        } else if ny < 0 {
            let grow = ny.abs();

            for _ in 0..grow {
                let mut row = VecDeque::new();
                row.resize(self.width(), Node::Clean);
                self.memory.push_front(row);
            }

            self.origin.0 += grow as usize;
        }

        let ny = self.origin.0 as isize + y;

        if nx >= self.width() as isize {
            let grow = nx - self.width() as isize + 1;

            self.memory.iter_mut().for_each(|row| for _ in 0..grow {
                row.push_back(Node::Clean)
            });
        } else if nx < 0 {
            let grow = nx.abs();

            self.memory.iter_mut().for_each(|row| for _ in 0..grow {
                row.push_front(Node::Clean)
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

#[derive(Clone, PartialEq, Eq, Debug)]
struct Carrier {
    position: (isize, isize),
    facing: advent::Facing,
    infected: usize,
}

impl Carrier {
    fn new() -> Carrier {
        Carrier {
            position: (0, 0),
            facing: advent::Facing::Up,
            infected: 0,
        }
    }

    fn step1(&mut self, memory: &mut Memory) {
        use Node::*;

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

    fn step2(&mut self, memory: &mut Memory) {
        use Node::*;

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

fn step1(input: &str) -> usize {
    let mut memory = Memory::parse(input);
    let mut carrier = Carrier::new();

    for _ in 0..10000 {
        carrier.step1(&mut memory);
    }

    carrier.infected
}

fn step2(input: &str) -> usize {
    let mut memory = Memory::parse(&input);
    let mut carrier = Carrier::new();

    for _ in 0..10000000 {
        carrier.step2(&mut memory);
    }

    carrier.infected
}

fn main() {
    let input = advent::download_input(2017, 22);

    println!("Step 1: {}", step1(&input));
    println!("Step 2: {}", step2(&input));
}
