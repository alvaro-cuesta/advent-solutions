//! # [Day 12: Digital Plumber](http://adventofcode.com/2017/day/12)
//!
//! Walking along the memory banks of the stream, you find a small village
//! that is experiencing a little confusion: some programs can't communicate
//! with each other.
//!
//! Programs in this village communicate using a fixed system of *pipes*.
//! Messages are passed between programs using these pipes, but most
//! programs aren't connected to each other directly. Instead, programs pass
//! messages between each other until the message reaches the intended
//! recipient.
//!
//! For some reason, though, some of these messages aren't ever reaching
//! their intended recipient, and the programs suspect that some <span
//! title="Yes, citizens, plumbing! It's the latest invention to hit Rome!">
//! pipes</span> are missing. They would like you to investigate.
//!
//! You walk through the village and record the ID of each program and the
//! IDs with which it can communicate directly (your puzzle input). Each
//! program has one or more programs with which it can communicate, and
//! these pipes are bidirectional; if `8` says it can communicate with `11`,
//! then `11` will say it can communicate with `8`.

use std::collections::{ HashMap, HashSet };
use ::parse::unsigned_number;

named!{ parse_connection (&[u8]) -> (usize, Vec<usize>),
    do_parse!(
        id: unsigned_number >>
        tag!(" <-> ") >>
        connected: separated_list!(tag!(", "), unsigned_number) >>
        (( id, connected ))
    )
}

named!( pub parse_connections (&[u8]) -> HashMap<usize, Vec<usize>>,
    fold_many0!(
        do_parse!(connection: parse_connection >> tag!("\n") >> (connection)),
        HashMap::new(),
        |mut map: HashMap<_ , _>, (id, connected)| {
            map.insert(id, connected);
            map
        }
    )
);

fn take_group(
    connections: &HashMap<usize, Vec<usize>>,
    remaining_ids: &mut HashSet<usize>,
    start_id: usize
) {
    let mut current_ids = vec![start_id];

    while let Some(id) = current_ids.pop() {
        for connected_to in connections[&id].iter()
            .filter(|&id| remaining_ids.contains(id))
            .collect::<Vec<_>>()
        {
            remaining_ids.remove(connected_to);
            current_ids.push(*connected_to);
        }
    }
}

/// You need to figure out how many programs are in the group that contains
/// program ID `0`.
///
/// For example, suppose you go door-to-door like a travelling salesman and
/// record the following list:
///
/// ```text
/// 0 <-> 2
/// 1 <-> 1
/// 2 <-> 0, 3, 4
/// 3 <-> 2, 4
/// 4 <-> 2, 3, 6
/// 5 <-> 6
/// 6 <-> 4, 5
/// ```
///
/// In this example, the following programs are in the group that contains
/// program ID `0`:
///
/// -   Program `0` by definition.
/// -   Program `2`, directly connected to program `0`.
/// -   Program `3` via program `2`.
/// -   Program `4` via program `2`.
/// -   Program `5` via programs `6`, then `4`, then `2`.
/// -   Program `6` via programs `4`, then `2`.
///
/// Therefore, a total of `6` programs are in this group; all but program
/// `1`, which has a pipe that connects it to itself.
///
/// ```
/// # use advent_solutions::advent2017::day12::{ parse_connections, part1 };
/// # let input = b"0 <-> 2
/// # 1 <-> 1
/// # 2 <-> 0, 3, 4
/// # 3 <-> 2, 4
/// # 4 <-> 2, 3, 6
/// # 5 <-> 6
/// # 6 <-> 4, 5
/// # ";
/// let connections = parse_connections(input)
///     .to_full_result()
///     .expect("Error parsing connections");
///
/// assert_eq!(part1(&connections), 6);
/// ```
///
/// *How many programs* are in the group that contains program ID `0`?
pub fn part1(connections: &HashMap<usize, Vec<usize>>) -> usize {
    let mut remaining_ids = connections.keys().cloned().collect::<HashSet<_>>();
    take_group(connections, &mut remaining_ids, 0);

    connections.len() - remaining_ids.len()
}

/// There are more programs than just the ones in the group containing
/// program ID `0`. The rest of them have no way of reaching that group, and
/// still might have no way of reaching each other.
///
/// A *group* is a collection of programs that can all communicate via pipes
/// either directly or indirectly. The programs you identified just a moment
/// ago are all part of the same group. Now, they would like you to
/// determine the total number of groups.
///
/// In the example above, there were `2` groups: one consisting of programs
/// `0,2,3,4,5,6`, and the other consisting solely of program `1`.
///
/// ```
/// # use advent_solutions::advent2017::day12::{ parse_connections, part2 };
/// let input = b"0 <-> 2
/// 1 <-> 1
/// 2 <-> 0, 3, 4
/// 3 <-> 2, 4
/// 4 <-> 2, 3, 6
/// 5 <-> 6
/// 6 <-> 4, 5
/// ";
///
/// let connections = parse_connections(input)
///     .to_full_result()
///     .expect("Error parsing connections");
///
/// assert_eq!(part2(&connections), 2);
/// ```
///
/// *How many groups are there* in total?
pub fn part2(connections: &HashMap<usize, Vec<usize>>) -> usize {
    let mut remaining_ids = connections.keys().cloned().collect::<HashSet<_>>();
    let mut groups = 0;

    while let Some(&start_id) = (&remaining_ids).into_iter().next() {
        take_group(connections, &mut remaining_ids, start_id);
        groups += 1;
    }

    groups
}

pub fn parse_input(input: &str) -> HashMap<usize, Vec<usize>> {
    parse_connections(input.as_bytes())
        .to_full_result()
        .expect("Error parsing connections")
}

test_day!("12", 175, 213);
