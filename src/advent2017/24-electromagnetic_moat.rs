use ::parse::unsigned_number;

pub type Component = (usize, usize);

named!{ parse_component (&[u8]) -> Component,
    do_parse!(
        a: unsigned_number >>
        char!('/') >>
        b: unsigned_number >>

        ((a, b))
    )
}

named!{ parse_components (&[u8]) -> Vec<Component>,
    lines!(parse_component)
}

pub fn part1(_: &Vec<Component>) -> usize {
    0
}

pub fn part2(_: &Vec<Component>) -> usize {
    0
}

pub fn parse_input(input: &str) -> Vec<Component> {
    parse_components(input.as_bytes())
        .to_full_result()
        .expect("Error parsing components")
}

test_day!("24", 0, 0);
