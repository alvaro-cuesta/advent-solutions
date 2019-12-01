//! # [Day 24: Electromagnetic Moat](http://adventofcode.com/2017/day/24)
//!
//! The CPU itself is a large, black building surrounded by a bottomless
//! pit. Enormous metal tubes extend outward from the side of the building
//! at regular intervals and descend down into the void. There's no way to
//! cross, but you need to get inside.
//!
//! No way, of course, other than building a *bridge* out of the magnetic
//! components strewn about nearby.

use itertools::Itertools;
use ::parse::unsigned_number;

/// Each component has two *ports*, one on each end. The ports come in all
/// different types, and only matching types can be connected. You take an
/// inventory of the components by their port types (your puzzle input).
/// Each port is identified by the number of *pins* it uses; more pins mean
/// a stronger connection for your bridge. A `3/7` component, for example,
/// has a type-`3` port on one side, and a type-`7` port on the other.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Component(usize, usize);

impl Component {
    named!{ from_bytes (&[u8]) -> Component,
        do_parse!(
            a: unsigned_number >>
            char!('/') >>
            b: unsigned_number >>

            (Component(a, b))
        )
    }

    named!{ list_from_bytes (&[u8]) -> Vec<Component>,
        lines!(Component::from_bytes)
    }

    fn strength(self) -> usize {
        self.0 + self.1
    }
}

fn dfs_step(
    path: &mut Vec<usize>,
    last_index: usize,
    graph: &Vec<Component>,
    cmp_f: &dyn Fn((usize, usize), (usize, usize)) -> bool,
) -> (usize, usize)
{
    let edges = graph.iter()
        .positions(|&Component(a, b)| last_index == a || last_index == b)
        .filter(|e| !path.contains(e))
        .collect::<Vec<_>>();

    if edges.len() == 0 {
        let strength = path.iter()
            .map(|&ei| graph[ei].strength())
            .sum::<usize>();

        return (path.len(), strength);
    }

    let mut max_len = 0;
    let mut max_strength = 0;

    for edge in edges {
        path.push(edge);

        let new_index = if graph[edge].0 == last_index {
            graph[edge].1
        } else {
            graph[edge].0
        };

        let (new_len, new_strength) = dfs_step(path, new_index, graph, cmp_f);

        if cmp_f((max_len, max_strength), (new_len, new_strength)) {
            max_len = new_len;
            max_strength = new_strength;
        }

        path.pop();
    }

    (max_len, max_strength)
}

/// Your side of the pit is metallic; a perfect surface to connect a
/// magnetic, *zero-pin port*. Because of this, the first port you use must
/// be of type `0`. It doesn't matter what type of port you end with; your
/// goal is just to make the bridge as strong as possible.
fn dfs(
    graph: &Vec<Component>,
    cmp_f: &dyn Fn((usize, usize), (usize, usize)) -> bool,
) -> usize
{
    dfs_step(&mut vec![], 0, graph, cmp_f).1
}


/// The *strength* of a bridge is the sum of the port types in each
/// component. For example, if your bridge is made of components `0/3`,
/// `3/7`, and `7/4`, your bridge has a strength of `0+3 + 3+7 + 7+4 = 24`.
///
/// For example, suppose you had the following components:
///
/// 0/2
/// 2/2
/// 2/3
/// 3/4
/// 3/5
/// 0/1
/// 10/1
/// 9/10
///
/// With them, you could make the following valid bridges:
///
/// -   `0/1`
/// -   `0/1`--`10/1`
/// -   `0/1`--`10/1`--`9/10`
/// -   `0/2`
/// -   `0/2`--`2/3`
/// -   `0/2`--`2/3`--`3/4`
/// -   `0/2`--`2/3`--`3/5`
/// -   `0/2`--`2/2`
/// -   `0/2`--`2/2`--`2/3`
/// -   `0/2`--`2/2`--`2/3`--`3/4`
/// -   `0/2`--`2/2`--`2/3`--`3/5`
///
/// (Note how, as shown by `10/1`, order of ports within a component doesn't
/// matter. However, you may only use each port on a component once.)
///
/// Of these bridges, the *strongest* one is `0/1`--`10/1`--`9/10`; it has a
/// strength of `0+1 + 1+10 + 10+9 = 31`.
///
/// ```
/// # use advent_solutions::advent2017::day24::{ parse_input, part1 };
/// let input = parse_input("\
/// 0/2
/// 2/2
/// 2/3
/// 3/4
/// 3/5
/// 0/1
/// 10/1
/// 9/10
/// ");
///
/// assert_eq!(part1(&input), 31);
/// ```
///
/// *What is the strength of the strongest bridge you can make* with the
/// components you have available?
pub fn part1(graph: &Vec<Component>) -> usize {
    dfs(
        graph,
        &|(_, max_strength), (_, new_strength)| new_strength > max_strength
    )
}

/// The bridge you've built isn't long enough; you can't <span
/// title="Who do you think you are, Mario?">jump the rest of the way</span>.
///
/// In the example above, there are two longest bridges:
///
/// -   `0/2`--`2/2`--`2/3`--`3/4`
/// -   `0/2`--`2/2`--`2/3`--`3/5`
///
/// Of them, the one which uses the `3/5` component is stronger; its
/// strength is `0+2 + 2+2 + 2+3 + 3+5 = 19`.
///
/// ```
/// # use advent_solutions::advent2017::day24::{ parse_input, part2 };
/// let input = parse_input("\
/// 0/2
/// 2/2
/// 2/3
/// 3/4
/// 3/5
/// 0/1
/// 10/1
/// 9/10
/// ");
///
/// assert_eq!(part2(&input), 19);
/// ```
///
/// *What is the strength of the longest bridge you can make?* If you can
/// make multiple bridges of the longest length, pick the *strongest* one.
pub fn part2(graph: &Vec<Component>) -> usize {
    dfs(
        graph,
        &|(max_len, max_strength), (new_len, new_strength)|
            new_len > max_len || (new_len == max_len && new_strength > max_strength)
    )
}

pub fn parse_input(input: &str) -> Vec<Component> {
    Component::list_from_bytes(input.as_bytes())
        .to_full_result()
        .expect("Error parsing components")
}

test_day!("24", 1868, 1841);
