extern crate advent;
#[macro_use] extern crate nom;

use std::collections::{ HashMap, HashSet };
use std::str::FromStr;
use nom::digit;

named!{ parse_id (&[u8]) -> usize,
    map_res!(
        map_res!(
            digit,
            std::str::from_utf8
        ),
        FromStr::from_str
    )
}

named!{ parse_connection (&[u8]) -> (usize, Vec<usize>),
    do_parse!(
        id: parse_id >>
        tag!(" <-> ") >>
        connected: separated_list!(tag!(", "), parse_id) >>
        (( id, connected ))
    )
}

named!( parse_connections (&[u8]) -> HashMap<usize, Vec<usize>>,
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

fn step1(connections: &HashMap<usize, Vec<usize>>) -> usize {
    let mut remaining_ids = connections.keys().cloned().collect::<HashSet<_>>();
    take_group(connections, &mut remaining_ids, 0);

    connections.len() - remaining_ids.len()
}

fn step2(connections: &HashMap<usize, Vec<usize>>) -> usize {
    let mut remaining_ids = connections.keys().cloned().collect::<HashSet<_>>();
    let mut groups = 0;

    while let Some(&start_id) = (&remaining_ids).into_iter().next() {
        take_group(connections, &mut remaining_ids, start_id);
        groups += 1;
    }

    groups
}

fn main() {
    let input = advent::download_input(2017, 12);

    let connections = parse_connections(input.as_bytes())
            .to_full_result()
            .expect("Error parsing connections");

    println!("Step 1: {}", step1(&connections));
    println!("Step 2: {}", step2(&connections));
}
