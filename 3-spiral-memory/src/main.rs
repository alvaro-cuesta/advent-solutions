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
    std::iter::once(&(0isize, 0isize))
    .chain(
        (1..).interleave(1..)
        .zip(DIRECTIONS.iter().cycle())
        .flat_map(|(len, dir)| std::iter::repeat(dir).take(len))
    )
}

use std::collections::HashMap;

fn main() {
    let mut input = advent::download_input(2017, 3);
    input.pop();

    let index = input.parse::<usize>()
        .expect("Unexpected non-integer");

    let (x, y) = spiral()
        .take(index - 1)
        .fold((0, 0), |(x, y), &(dx, dy)| {
            (x + dx, y + dy)
        });

    println!("Step 1: {}", x.abs() + y.abs());

    let mut cache = HashMap::new();
    cache.insert((0, 0), 1);

    let step2 = spiral()
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
        .expect("Found no solution for step 2");

    println!("Step 2: {}", step2);
}
