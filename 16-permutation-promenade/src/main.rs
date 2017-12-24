extern crate advent;
#[macro_use] extern crate nom;

use std::str::FromStr;
use nom::{ anychar, digit };

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Move {
    fn apply(&self, mut programs: Vec<char>) -> Vec<char> {
        use Move::*;

        match *self {
            Spin(length) => {
                let total = programs.len();
                let mut end = programs.split_off(total - length);
                end.extend(programs);
                end
            },
            Exchange(a, b) => {
                programs.swap(a, b);
                programs
            },
            Partner(a, b) => {
                let a_i = programs.iter().position(|&x| x == a).unwrap();
                let b_i = programs.iter().position(|&x| x == b).unwrap();
                programs.swap(a_i, b_i);
                programs
            },
        }
    }
}

named!{ parse_number (&[u8]) -> usize,
    map_res!(
        map_res!(digit, std::str::from_utf8),
        FromStr::from_str
    )
}

named!{ parse_spin (&[u8]) -> Move,
    do_parse!(
        char!('s') >>
        x: parse_number >>

        (Move::Spin(x))
    )
}

named!{ parse_exchange (&[u8]) -> Move,
    do_parse!(
        char!('x') >>
        a: parse_number >>
        char!('/') >>
        b: parse_number >>

        (Move::Exchange(a, b))
    )
}

named!{ parse_partner (&[u8]) -> Move,
    do_parse!(
        char!('p') >>
        a: anychar >>
        char!('/') >>
        b: anychar >>

        (Move::Partner(a, b))
    )
}

named!{ parse_moves (&[u8]) -> Vec<Move>,
    separated_list!(
        char!(','),
        alt!(parse_spin | parse_exchange | parse_partner)
    )
}

fn initial_programs() -> Vec<char> {
    vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ]
}

fn dance(mut programs: Vec<char>, moves: &[Move]) -> Vec<char> {
    for m in moves {
        programs = m.apply(programs);
    }

    programs
}

fn step1(moves: &[Move]) -> String {
    dance(initial_programs(), moves)
        .into_iter().collect()
}

fn find_cycle(moves: &[Move]) -> usize {
    let programs = initial_programs();
    let mut p = initial_programs();

    for i in 1.. {
        p = dance(p, moves);

        if programs == p {
            return i;
        }
    }

    unreachable!();
}

fn step2(moves: &[Move]) -> String {
    let dance_times = 1_000_000_000 % find_cycle(moves);
    println!("{}", dance_times);

    let mut programs = initial_programs();

    for _ in 0..dance_times {
        programs = dance(programs, moves);
    }

    programs.into_iter().collect()
}

fn main() {
    let input = advent::download_input(2017, 16);

    let moves = parse_moves(input.as_bytes())
            .to_full_result()
            .expect("Error parsing moves");

    println!("Step 1: {}", step1(&moves));
    println!("Step 2: {}", step2(&moves));
}

#[cfg(test)]
mod tests {
    #[test]
    fn spin() {
        assert_eq!(
            ::Move::Spin(3).apply(vec!['a', 'b', 'c', 'd', 'e']),
            ['c', 'd', 'e', 'a', 'b']
        )
    }
}
