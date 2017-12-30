use std::collections::HashMap;

type Grid = Vec<Vec<bool>>;
type Rule = (Grid, Grid);

fn char_to_bool(x: char) -> bool { x == '#' }

named!{ parse_cell (&[u8]) -> Vec<bool>,
    many1!(
        map!(
            alt!(char!('#') | char!('.')),
            char_to_bool
        )
    )
}

named!{ parse_grid (&[u8]) -> Grid,
    separated_list!(char!('/'), parse_cell)
}

named!{ parse_rule (&[u8]) -> Rule,
    do_parse!(
        l: parse_grid >>
        tag!(" => ") >>
        r: parse_grid >>

        (l, r)
    )
}

named!{ parse_rules (&[u8]) -> Vec<Rule>,
    lines!(parse_rule)
}

fn and_rotations(shape: &Grid) -> Vec<Grid> {
    let mut result = vec![shape.clone()];

    let size = shape.len();

    let mut swap = shape.clone();
    let mut flip_y = shape.clone();
    let mut flip_y_swap = shape.clone();
    let mut flip_x = shape.clone();
    let mut flip_x_swap = shape.clone();
    let mut flip_both = shape.clone();
    let mut flip_both_swap = shape.clone();

    for y in 0..size {
        for x in 0..size {
            swap[y][x] = shape[x][y];
        }
    }

    for y in 0..size {
        for x in 0..size {
            flip_y[y][x] = shape[size - y - 1][x];
        }
    }

    for y in 0..size {
        for x in 0..size {
            flip_y_swap[y][x] = shape[x][size - y - 1];
        }
    }

    for y in 0..size {
        for x in 0..size {
            flip_x[y][x] = shape[y][size - x - 1];
        }
    }

    for y in 0..size {
        for x in 0..size {
            flip_x_swap[y][x] = shape[size - x - 1][y];
        }
    }

    for y in 0..size {
        for x in 0..size {
            flip_both[y][x] = shape[size - y - 1][size - x - 1];
        }
    }

    for y in 0..size {
        for x in 0..size {
            flip_both_swap[y][x] = shape[size - x - 1][size - y - 1];
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

fn print_grid(grid: &Grid) {
    let size = grid.len();

    for y in 0..size {
        for x in 0..size {
            print!("{}", if grid[y][x] { '#' } else { '.' });
        }

        println!("");
    }
}

fn grid_region(grid: &Grid, x: usize, y: usize, w: usize, h: usize) -> Grid {
    (0..h)
    .map(|yy| (0..w)
        .map(|xx| grid[y + yy][x + xx])
        .collect()
    )
    .collect()
}

fn split_grid(grid: &Grid) -> Vec<Vec<Grid>> {
    let size = grid.len();

    if size % 2 == 0 {
        let tiles = size / 2;

        (0..tiles)
        .map(|y| (0..tiles)
            .map(|x| grid_region(grid, x * 2, y * 2, 2, 2))
            .collect()
        )
        .collect()
    } else {
        let tiles = size / 3;

        (0..tiles)
        .map(|y| (0..tiles)
            .map(|x| grid_region(grid, x * 3, y * 3, 3, 3))
            .collect()
        )
        .collect()
    }
}

fn merge_grid(tiles: Vec<Vec<Grid>>) -> Grid {
    let num_tiles = tiles.len();
    let size = tiles[0][0].len();

    let mut result = vec![];

    for y_tile in 0..num_tiles {
        for y in 0..size {
            let mut row = vec![];

            for x_tile in 0..num_tiles {
                for x in 0..size {
                    row.push(tiles[y_tile][x_tile][y][x]);
                }
            }

            result.push(row);
        }
    }

    result
}

pub fn part1(input: &HashMap<Grid, Grid>) -> usize {
    println!("{:?}", input);

    {
        /*let test_grid = parse_grid(b".#./..#/###")
            .to_full_result()
            .unwrap();*/

        let test_grid = input.iter().next().unwrap().0;

        print_grid(&test_grid);

        println!("\nRotations:\n");
        for rotation in and_rotations(test_grid) {
            print_grid(&rotation);
            println!("");
        }

        println!("\n0,0 to 2,2:");
        print_grid(&grid_region(&test_grid, 0, 0, 2, 2));

        println!("\nSplit");

        for y in &split_grid(&test_grid) {
            for x in y {
                print_grid(x);
                println!("");
            }
        }

        println!("Merged:");

        print_grid(&merge_grid(split_grid(&test_grid)));
        println!("");
    }

    0
}

pub fn part2(input: &HashMap<Grid, Grid>) -> usize {
    0
}

pub fn parse_input(input: &str) -> HashMap<Grid, Grid> {
    parse_rules(/*input.as_bytes())*/b"###../.#.../....#/#...#/..... => ././.
")
        .to_full_result()
        .expect("Error parsing rules")
        .into_iter()
        .collect()
}

test_day!("21", 0, 0);
