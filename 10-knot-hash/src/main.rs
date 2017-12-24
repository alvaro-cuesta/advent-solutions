#![feature(conservative_impl_trait)]

extern crate knot_hash;

fn step1(input: &str) -> u16 {
    let lengths = input
        .split(',')
        .map(|l| l.parse::<u8>().expect("Unexpected non-u8 length"))
        .collect::<Vec<_>>();

    let mut hash = knot_hash::hash_lengths(&lengths, 1);

    (hash[0] as u16) * (hash[1] as u16)
}

fn step2(input: &str) -> String {
    knot_hash::hash_str(input, 64).iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>()
}

fn main() {
    let input = advent::download_single_input(2017, 10);

    println!("Step 1: {}", step1(&input));
    println!("Step 2: {}", step2(&input));
}

#[cfg(test)]
mod test {
    #[test]
    fn empty() {
        assert_eq!(&::step2(""), "a2582a3a0e66e6e86e3812dcb672a272");
    }

    #[test]
    fn aoc2017() {
        assert_eq!(&::step2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    }

    #[test]
    fn one_two_three() {
        assert_eq!(&::step2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    }

    #[test]
    fn one_two_four() {
        assert_eq!(&::step2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
