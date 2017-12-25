extern crate advent;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Cell {
    Road,
    Empty,
    Letter(char),
}

type Grid = Vec<Vec<Cell>>;

impl From<char> for Cell {
    fn from(c: char) -> Self {
        use Cell::*;

        match c {
            '-' | '|' | '+' => Road,
            ' ' => Empty,
            c => Letter(c),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Packet {
    position: (usize, usize),
    direction: advent::Facing,
    collected: Vec<char>,
}

impl Packet {
    fn new(x: usize) -> Packet {
        Packet {
            position: (x, 0),
            direction: advent::Facing::Down,
            collected: Vec::new(),
        }
    }

    fn step(&mut self, grid: &Grid) -> bool {
        if let Cell::Letter(c) = grid[self.position.1][self.position.0] {
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

    fn neighbors(&self, grid: &Grid) -> Vec<advent::Facing> {
        use advent::Facing::*;

        let w = grid[0].len();
        let h = grid.len();

        [Up, Down, Left, Right].iter()
            .map(|&dir| (dir, self.position + dir))
            .filter(|&(_, (x, y))|
                x > 0 && y > 0
                && x < w && y < h
                && grid[y as usize][x as usize] != Cell::Empty
            )
            .map(|(dir, _)| dir)
            .collect()
    }
}

fn main() {
    let grid: Grid = advent::download_input(2017, 19)
        .lines()
        .map(|line| line.chars().map(Into::<Cell>::into).collect())
        .collect();

    let x = grid[0].iter().position(|&c| c == Cell::Road)
        .expect("Could not find starting point");

    let mut packet = Packet::new(x);
    let mut steps = 1;

    while packet.step(&grid) {
        steps += 1;
    }

    println!("Step 1: {}", packet.collected.into_iter().collect::<String>());
    println!("Step 2: {}", steps);
}
