#![feature(conservative_impl_trait)]

extern crate advent;
extern crate itertools;

use itertools::Itertools;

static DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
static NEIGHBORS: [(isize, isize); 8] = [
    (-1, 1),  (0, 1),  (1, 1),
    (-1, 0),           (1, 0),
    (-1, -1), (0, -1), (1, -1),
];

fn spiral() -> impl Iterator<Item=&'static (isize, isize)> {
    (1..).interleave(1..)
    .zip(DIRECTIONS.iter().cycle())
    .flat_map(|(len, dir)| std::iter::repeat(dir).take(len))
}

fn step1(index: usize) -> usize {
    let (x, y) = spiral()
        .take(index - 1)
        .fold((0, 0), |(x, y), &(dx, dy)| {
            (x + dx, y + dy)
        });

    (x.abs() + y.abs()) as usize
}

fn step2(index: usize) -> usize {
    use std::collections::HashMap;

    let mut cache = HashMap::new();
    cache.insert((0, 0), 1);

    spiral()
        .scan(((0, 0), cache), |state, &(dx, dy)| {
            (state.0).0 += dx;
            (state.0).1 += dy;

            let val = std::cmp::max(
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
        .skip_while(|&x| x < index)
        .next()
        .expect("Found no solution for step 2")
}

fn main() {
    let index = advent::download_single_input(2017, 3)
        .parse::<usize>()
        .expect("Unexpected non-integer");

    println!("Step 1: {}", step1(index));
    println!("Step 2: {}", step2(index));
}
