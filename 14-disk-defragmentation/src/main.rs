extern crate advent;
extern crate knot_hash;

use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Bits {
    v: usize,
    length: usize,
    i: usize,
}

impl Bits {
    fn new(v: usize, length: usize) -> Bits {
        Bits { v, length, i: 0 }
    }
}

impl Iterator for Bits {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.length {
            return None;
        }

        let bit: usize = 1 << (self.length - self.i - 1);

        self.i += 1;

        return Some((self.v & bit) > 0)
    }
}

struct Grid(Vec<Vec<bool>>);

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter()
            .map(|line| {
                line.iter().map(|&x| write!(f, "{}", if x { '#' } else { '.' }))
                    .skip_while(|x| x.is_ok())
                    .next()
                    .unwrap_or(Ok(()))
                    .and_then(|_| write!(f, "\n"))
            })
            .skip_while(|x| x.is_ok())
            .next()
            .unwrap_or(Ok(()))
            .and_then(|_| write!(f, "\n"))
    }
}

fn make_hashes(input: &str) -> Vec<Vec<u8>> {
    (0..128)
        .map(|i| {
            let i_str = format!("{}-{}", input, i);
            knot_hash::hash_str(&i_str)
        })
        .collect::<Vec<_>>()
}

fn step1<'a, I, J>(hashes: I) -> u32
    where I: IntoIterator<Item=J>,
          J: IntoIterator<Item=&'a u8>,
{
    hashes.into_iter()
        .map(|row| row.into_iter()
            .map(|x| x.count_ones())
            .sum::<u32>()
        )
        .sum::<u32>()
}

fn step2<'a, I, J>(hashes: I) -> usize
    where I: IntoIterator<Item=J>,
          J: IntoIterator<Item=&'a u8>,
{
    let grid = Grid(
        hashes.into_iter()
            .map(|row| row.into_iter()
                .flat_map(|x| Bits::new(*x as usize, 8))
                .collect()
            )
            .collect()
    );

    use std::collections::HashSet;

    let mut ungrouped = grid.0.iter()
        .enumerate()
        .flat_map(|(y, row)| std::iter::repeat(y)
            .zip(0..row.len())
        )
        .filter(|&(y, x)| grid.0[y][x])
        .collect::<HashSet<(usize, usize)>>();

    let mut groups = Vec::new();

    while let Some(&(y, x)) = ungrouped.iter().next() {
        let mut group = Vec::new();

        let mut candidates = vec![(y, x)];

        while let Some((y, x)) = candidates.pop() {
            if ungrouped.contains(&(y, x)) {
                group.push((y, x));

                if y < 127 { candidates.push((y + 1, x)); }
                if y > 0   { candidates.push((y - 1, x)); }
                if x < 127 { candidates.push((y, x + 1)); }
                if x > 0   { candidates.push((y, x - 1)); }

                ungrouped.remove(&(y, x));
            }
        }

        groups.push(group);
    }

    groups.len()
}

fn main() {
    let hashes = make_hashes(&advent::download_single_input(2017, 14));

    println!("Step 1: {}", step1(&hashes));
    println!("Step 2: {}", step2(&hashes));
}

#[cfg(test)]
mod tests {
    #[test]
    fn step1() {
        let hashes = ::make_hashes("flqrgnkx");

        assert_eq!(::step1(&hashes), 8108);
    }

    #[test]
    fn step2() {
        let hashes = ::make_hashes("flqrgnkx");

        assert_eq!(::step2(&hashes), 1242);
    }
}
