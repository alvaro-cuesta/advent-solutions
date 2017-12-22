#![feature(try_from)]

extern crate advent;
extern crate itertools;

use std::fmt;
use std::convert::TryFrom;
use std::collections::VecDeque;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Node {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl TryFrom<char> for Node {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Node::Clean),
            'W' => Ok(Node::Weakened),
            '#' => Ok(Node::Infected),
            'F' => Ok(Node::Flagged),
            _ => Err("Invalid node character"),
        }
    }
}

impl Into<char> for Node {
    fn into(self) -> char {
        match self {
            Node::Clean => '.',
            Node::Weakened => 'W',
            Node::Infected => '#',
            Node::Flagged => 'F',
        }
    }
}

impl<'a> Into<char> for &'a Node {
    fn into(self) -> char {
        match *self {
            Node::Clean => '.',
            Node::Weakened => 'W',
            Node::Infected => '#',
            Node::Flagged => 'F',
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<char>::into(self))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    fn ccw(&self) -> Facing {
        use Facing::*;

        match *self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }

    fn cw(&self) -> Facing {
        use Facing::*;

        match *self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }

    fn reverse(&self) -> Facing {
        use Facing::*;

        match *self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

impl Into<(isize, isize)> for Facing {
    fn into(self) -> (isize, isize) {
        match self {
            Facing::Up => (0, -1),
            Facing::Down => (0, 1),
            Facing::Left => (-1, 0),
            Facing::Right => (1, 0),
        }
    }
}

fn step((x, y): (isize, isize), facing: Facing) -> (isize, isize) {
    let (dx, dy) = facing.into();
    (x + dx, y + dy)
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

fn main() {
    let input = advent::download_input(2017, 22);

    let mut memory = Memory::parse(&input);
    let mut position = (0, 0);
    let mut facing = Facing::Up;
    let mut infected = 0;

    for _ in 0..10000 {
        match memory[position] {
            Node::Clean => {
                facing = facing.ccw();
                memory[position] = Node::Infected;
                infected += 1;
            },
            Node::Infected => {
                facing = facing.cw();
                memory[position] = Node::Clean;
            },
            _ => panic!("Step 1 should not have weakened or flagged nodes"),
        };

        position = step(position, facing);
    }

    println!("Step 1: {}", infected);

    let mut memory = Memory::parse(&input);
    let mut position = (0, 0);
    let mut facing = Facing::Up;
    let mut infected = 0;

    for _ in 0..10000000 {
        match memory[position] {
            Node::Clean => {
                facing = facing.ccw();
                memory[position] = Node::Weakened;
            },
            Node::Weakened => {
                memory[position] = Node::Infected;
                infected += 1;
            },
            Node::Infected => {
                facing = facing.cw();
                memory[position] = Node::Flagged;
            },
            Node::Flagged => {
                facing = facing.reverse();
                memory[position] = Node::Clean;
            },
        };

        position = step(position, facing);
    }

    println!("Step 2: {}", infected);
}
