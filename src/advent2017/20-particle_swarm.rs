use ::parse::signed_number;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Particle {
    p: (isize, isize, isize),
    v: (isize, isize, isize),
    a: (isize, isize, isize),
}

named!{ parse_vector (&[u8]) -> (isize, isize, isize),
    do_parse!(
        char!('<') >>
        x: signed_number >>
        char!(',') >>
        y: signed_number >>
        char!(',') >>
        z: signed_number >>
        char!('>') >>

        ((x, y, z))
    )
}


named!{ parse_particle (&[u8]) -> Particle,
    do_parse!(
        tag!("p=") >>
        p: parse_vector >>
        tag!(", v=") >>
        v: parse_vector >>
        tag!(", a=") >>
        a: parse_vector >>

        (Particle { p, v, a })
    )
}

named!{ parse_particles (&[u8]) -> Vec<Particle>,
    separated_list_complete!(tag!("\n"), parse_particle)
}

pub fn part1(particles: &Vec<Particle>) -> usize {
    let mut by_accel = particles.iter()
        .enumerate()
        .collect::<Vec<_>>();

    by_accel.sort_by_key(|&(_, p)|
          p.a.0 * p.a.0
        + p.a.1 * p.a.1
        + p.a.2 * p.a.2
    );

    by_accel[0].0
}

pub fn part2(particles: &Vec<Particle>) -> usize {
    unimplemented!()
}

pub fn parse_input(input: &str) -> Vec<Particle> {
    parse_particles(input.as_bytes())
        .to_full_result()
        .expect("Error parsing particles")
}

test_day!("20", 300, 0);
