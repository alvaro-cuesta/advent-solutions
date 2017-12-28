//! # [Day 20: Particle Swarm](http://adventofcode.com/2017/day/20)
//!
//! Suddenly, the GPU contacts you, asking for <span title="...as if millions
//! of graphics pipelines suddenly cried out for help, but suddenly started
//! working on something else instead because they all have to do the same
//! thing at the same time and can't spend very long asking for help.">help
//! </span>. Someone has asked it to simulate *too many particles*, and it
//! won't be able to finish them all in time to render the next frame at this
//! rate.

use std::cmp::min;
use ::parse::signed_number;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Particle {
    p: (isize, isize, isize),
    v: (isize, isize, isize),
    a: (isize, isize, isize),
}

named!{ parse_vector (&[u8]) -> (isize, isize, isize),
    do_parse!(
        char!('<') >>
        x: ws!(signed_number) >>
        char!(',') >>
        y: ws!(signed_number) >>
        char!(',') >>
        z: ws!(signed_number) >>
        char!('>') >>

        ((x as isize, y as isize, z as isize))
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

/// Each tick, all particles are updated simultaneously. A particle's
/// properties are updated in the following order:
///
/// -   Increase the `X` velocity by the `X` acceleration.
/// -   Increase the `Y` velocity by the `Y` acceleration.
/// -   Increase the `Z` velocity by the `Z` acceleration.
/// -   Increase the `X` position by the `X` velocity.
/// -   Increase the `Y` position by the `Y` velocity.
/// -   Increase the `Z` position by the `Z` velocity.
///
/// Because of seemingly tenuous rationale involving [z-buffering], the GPU
/// would like to know which particle will stay closest to position
/// `<0,0,0>` in the long term. Measure this using the [Manhattan distance],
/// which in this situation is simply the sum of the absolute values of a
/// particle's `X`, `Y`, and `Z` position.
///
/// For example, suppose you are only given two particles, both of which
/// stay entirely on the X-axis (for simplicity). Drawing the current states
/// of particles `0` and `1` (in that order) with an adjacent a number line
/// and diagram of current `X` positions (marked in parenthesis), the
/// following would take place:
///
/// ```text
/// p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
/// p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>                         (0)(1)
///
/// p=< 4,0,0>, v=< 1,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
/// p=< 2,0,0>, v=<-2,0,0>, a=<-2,0,0>                      (1)   (0)
///
/// p=< 4,0,0>, v=< 0,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
/// p=<-2,0,0>, v=<-4,0,0>, a=<-2,0,0>          (1)               (0)
///
/// p=< 3,0,0>, v=<-1,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
/// p=<-8,0,0>, v=<-6,0,0>, a=<-2,0,0>                         (0)
/// ```
///
/// At this point, particle `1` will never be closer to `<0,0,0>` than
/// particle `0`, and so, in the long run, particle `0` will stay closest.
///
/// ```
/// # use advent_solutions::advent2017::day20::{ parse_input, part1 };
/// # let input = parse_input("p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
/// # -4 -3 -2 -1  0  1  2  3  4
/// # p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>
/// # ");
/// assert_eq!(part1(&input), 0);
/// ```
///
/// *Which particle will stay closest to position `<0,0,0>`* in the long
/// term?
///
///   [z-buffering]: https://en.wikipedia.org/wiki/Z-buffering
///   [Manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
pub fn part1(particles: &Vec<Particle>) -> usize {
    let mut by_accel = particles.iter()
        .enumerate()
        .collect::<Vec<_>>();

    by_accel.sort_by_key(|&(_, p)|
          (p.a.0 * p.a.0
        + p.a.1 * p.a.1
        + p.a.2 * p.a.2) as u32
    );

    by_accel[0].0
}


#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Roots {
    None,
    One(isize),
    Two(isize, isize),
    Any
}

impl Roots {
    fn solve_quadratic(a: isize, b: isize, c: isize) -> Roots {
        match (a, b, c) {
            (0, 0, 0) => return Roots::Any,
            (0, 0, _) => return Roots::None,
            (0, _, _) => if c % b == 0 && -c / b >= 0 {
                return Roots::One(-c / b);
            } else {
                return Roots::None;
            },
            _ => {},
        };

        let determinant = b * b - 4 * a * c;
        if determinant < 0 {
            return Roots::None;
        }

        let sqrt = (determinant as f64).sqrt() as isize;
        if sqrt * sqrt != determinant {
            return Roots::None;
        }

        let r1 = -b + sqrt;
        let r2 = -b - sqrt;

        match (
            if r1 * a >= 0 && r1 % (2*a) == 0 { Some(r1 / (2*a)) } else { None },
            if r2 * a >= 0 && r2 % (2*a) == 0 { Some(r2 / (2*a)) } else { None },
        ) {
            (Some(a), Some(b)) if a == b => Roots::One(a),
            (Some(a), Some(b)) => Roots::Two(a, b),
            (Some(a), None) | (None, Some(a)) => Roots::One(a),
            (None, None) => Roots::None,
        }
    }
}

fn combine_roots(x: Roots, y: Roots) -> Roots {
    match (x, y) {
        (Roots::Any, x) | (x, Roots::Any) => x,

        (Roots::None, _) | (_, Roots::None) => Roots::None,

        (Roots::One(a), Roots::One(b))
            if a == b => Roots::One(a),

        (Roots::Two(a, b), Roots::One(c))
            if c == a || c == b => Roots::One(c),
        (Roots::One(a), Roots::Two(c, d))
            if a == c || a == d => Roots::One(a),

        (Roots::Two(a, b), Roots::Two(c, d))
            if (a == c && b == d) || (a == d && b == c) => Roots::Two(a, b),
        (Roots::Two(a, _), Roots::Two(c, d))
            if a == c || a == d => Roots::One(a),
        (Roots::Two(_, b), Roots::Two(c, d))
            if b == c || b == d => Roots::One(b),

        _ => Roots::None
    }
}

/// To simplify the problem further, the GPU would like to remove any
/// particles that *collide*. Particles collide if their positions ever
/// *exactly match*. Because particles are updated simultaneously, *more
/// than two particles* can collide at the same time and place. Once
/// particles collide, they are removed and cannot collide with anything
/// else after that tick.
///
/// For example:
///
/// p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
/// p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
/// p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>    (0)   (1)   (2)            (3)
/// p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>
///
/// p=<-3,0,0>, v=< 3,0,0>, a=< 0,0,0>
/// p=<-2,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
/// p=<-1,0,0>, v=< 1,0,0>, a=< 0,0,0>             (0)(1)(2)      (3)
/// p=< 2,0,0>, v=<-1,0,0>, a=< 0,0,0>
///
/// p=< 0,0,0>, v=< 3,0,0>, a=< 0,0,0>
/// p=< 0,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
/// p=< 0,0,0>, v=< 1,0,0>, a=< 0,0,0>                       X (3)
/// p=< 1,0,0>, v=<-1,0,0>, a=< 0,0,0>
///
/// ------destroyed by collision------
/// ------destroyed by collision------    -6 -5 -4 -3 -2 -1  0  1  2  3
/// ------destroyed by collision------                      (3)
/// p=< 0,0,0>, v=<-1,0,0>, a=< 0,0,0>
///
/// In this example, particles `0`, `1`, and `2` are simultaneously
/// destroyed at the time and place marked `X`. On the next tick, particle
/// `3` passes through unharmed.
///
/// ```
/// # use advent_solutions::advent2017::day20::{ parse_input, part2 };
/// # let input = parse_input("p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
/// # p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
/// # p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
/// # p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>
/// # ");
/// assert_eq!(part2(&input), 1);
/// ```
///
/// *How many particles are left* after all collisions are resolved?
pub fn part2(particles: &Vec<Particle>) -> usize {
    use itertools::Itertools;
    use std::collections::HashSet;

    let mut collisions = particles.iter()
        .enumerate()
        .combinations(2)
        .filter_map(|particles| {
            let (n0, p0) = particles[0];
            let (n1, p1) = particles[1];

            let rx = Roots::solve_quadratic(
                p0.a.0 - p1.a.0,
                2 * (p0.v.0 - p1.v.0) + (p0.a.0 - p1.a.0),
                2 * (p0.p.0 - p1.p.0),
            );

            let ry = Roots::solve_quadratic(
                p0.a.1 - p1.a.1,
                2 * (p0.v.1 - p1.v.1) + (p0.a.1 - p1.a.1),
                2 * (p0.p.1 - p1.p.1),
            );

            let rz = Roots::solve_quadratic(
                p0.a.2 - p1.a.2,
                2 * (p0.v.2 - p1.v.2) + (p0.a.2 - p1.a.2),
                2 * (p0.p.2 - p1.p.2),
            );

            match combine_roots(rx, combine_roots(ry, rz)) {
                Roots::None => None,
                Roots::One(n) => Some((n, n0, n1)),
                Roots::Two(a, b) => Some((min(a,b), n0, n1)),
                Roots::Any => Some((0, n0, n1))
            }
        })
        .collect::<Vec<_>>();

    collisions.sort_by_key(|&(t, _, _)| t);

    let mut alive_particles = (0..particles.len())
        .collect::<HashSet<_>>();

    for (_, group) in &collisions.iter().group_by(|elt| elt.0) {
        let alive_particles_copy = alive_particles.clone();

        for &(_, n0, n1) in group {
            if alive_particles_copy.contains(&n0) && alive_particles_copy.contains(&n1) {
                alive_particles.remove(&n0);
                alive_particles.remove(&n1);
            }
        }
    }

    alive_particles.len()
}

/// It transmits to you a buffer (your puzzle input) listing each particle
/// in order (starting with particle `0`, then particle `1`, particle `2`,
/// and so on). For each particle, it provides the `X`, `Y`, and `Z`
/// coordinates for the particle's position (`p`), velocity (`v`), and
/// acceleration (`a`), each in the format `<X,Y,Z>`.
pub fn parse_input(input: &str) -> Vec<Particle> {
    parse_particles(input.as_bytes())
        .to_full_result()
        .expect("Error parsing particles")
}

test_day!("20", 300, 502);
