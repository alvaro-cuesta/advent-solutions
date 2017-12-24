extern crate advent;

struct Generator {
    factor: u32,
    v: u32,
}

impl Generator {
    fn new(factor: u32, v: u32) -> Generator {
        Generator { factor, v }
    }
}

impl Iterator for Generator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
         self.v = (((self.v as u64) * (self.factor as u64)) % 0x7FFF_FFFF) as u32;
         Some(self.v)
    }
}

fn step1(a_f: u32, a_s: u32, b_f: u32, b_s: u32) -> usize {
    Generator::new(a_f, a_s)
        .zip(Generator::new(b_f, b_s))
        .take(40_000_000)
        .filter(|&(a, b)| (a & 0xFFFF) == (b & 0xFFFF))
        .count()
}

fn step2(a_f: u32, a_s: u32, b_f: u32, b_s: u32) -> usize {
    Generator::new(a_f, a_s)
        .filter(|x| x % 4 == 0)
        .zip(Generator::new(b_f, b_s)
            .filter(|x| x % 8 == 0)
        )
        .take(5_000_000)
        .filter(|&(a, b)| (a & 0xFFFF) == (b & 0xFFFF))
        .count()
}

fn main() {
    let input = advent::download_input(2017, 15);

    let mut starts = input.lines()
        .map(|line| line[24..].parse::<u32>().expect("Could not parse input"));

    let a_s = starts.next().expect("Could not find a_s");
    let b_s = starts.next().expect("Could not find b_s");
    assert!(starts.next().is_none(), "More than 2 lines");

    println!("Step 1: {}", step1(16807, a_s, 48271, b_s));
    println!("Step 2: {}", step2(16807, a_s, 48271, b_s));
}

#[cfg(test)]
mod tests {
    #[test]
    fn generators() {
        let a = ::Generator::new(16807, 65)
            .take(5)
            .collect::<Vec<_>>();
        let b = ::Generator::new(48271, 8921)
            .take(5)
            .collect::<Vec<_>>();

        assert_eq!(a, [1092455, 1181022009, 245556042, 1744312007, 1352636452]);
        assert_eq!(b, [430625591, 1233683848, 1431495498, 137874439, 285222916]);
    }

    #[test]
    fn step1() {
        assert_eq!(::step1(16807, 65, 48271, 8921), 588);
    }

    #[test]
    fn filtered_generators() {
        let a = ::Generator::new(16807, 65)
            .filter(|x| x % 4 == 0)
            .take(5)
            .collect::<Vec<_>>();
        let b = ::Generator::new(48271, 8921)
            .filter(|x| x % 8 == 0)
            .take(5)
            .collect::<Vec<_>>();

        assert_eq!(a, [1352636452, 1992081072, 530830436, 1980017072, 740335192]);
        assert_eq!(b, [1233683848, 862516352, 1159784568, 1616057672, 412269392]);
    }

    #[test]
    fn step2() {
        assert_eq!(::step2(16807, 65, 48271, 8921), 309);
    }
}
