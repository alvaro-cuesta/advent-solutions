use std::fmt;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Grid(Vec<Vec<bool>>);

fn char_to_bool(x: char) -> bool { x == '#' }

named!{ parse_cell (&[u8]) -> Vec<bool>,
    many1!(
        map!(
            alt!(char!('#') | char!('.')),
            char_to_bool
        )
    )
}

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

    // TODO: make unique
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

    named!{ from_bytes (&[u8]) -> Grid,
        map!(
            separated_list!(char!('/'), parse_cell),
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

fn merge_grid(tiles: Vec<Vec<Grid>>) -> Grid {
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

        // TODO: rotate and replace if match

        grid = merge_grid(split);
    }

    grid.0.into_iter()
        .map(|row| row.into_iter().filter(|&x| x).count())
        .sum::<usize>()
}

pub fn part1(input: &HashMap<Grid, Grid>) -> usize {
    solve(input, 5)
}

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
