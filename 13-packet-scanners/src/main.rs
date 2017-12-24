extern crate advent;

use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Layer {
    range: usize,
    position: usize,
    forward: bool,
}

impl Layer {
    fn step(&mut self) {
        if self.forward {
            if self.position < self.range - 1 {
                self.position += 1;
            } else {
                self.forward = false;
                self.position -= 1;
            }
        } else {
            if self.position > 0 {
                self.position -= 1;
            } else {
                self.forward = true;
                self.position += 1;
            }
        }
    }
}

impl FromStr for Layer {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Layer {
            range: s.parse::<usize>()?,
            position: 0,
            forward: true,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Firewall(Vec<Option<Layer>>);

impl Firewall {
    fn new(input: &str) -> Firewall {
        Firewall(
            input.lines()
            .fold(Vec::new(), |mut firewall, line| {
                let mut vals = line.split(": ");
                let depth: usize = vals.next().map(FromStr::from_str).unwrap().unwrap();
                let layer = vals.next().map(FromStr::from_str).unwrap().unwrap();

                if firewall.len() < depth + 1 {
                    firewall.resize(depth + 1, None);
                }

                firewall[depth] = Some(layer);
                firewall
            })
        )
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn step(&mut self) {
        for layer in &mut self.0 {
            layer.iter_mut().for_each(|x| x.step());
        }
    }
}

impl std::ops::Index<usize> for Firewall {
    type Output = Option<Layer>;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl std::ops::IndexMut<usize> for Firewall {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

fn step1(input: &str) -> usize {
    let mut firewall = Firewall::new(&input);
    let mut severity = 0;

    for depth in 0..firewall.len() {
        if let Some(layer) = firewall[depth] {
            if layer.position == 0 {
                severity += depth * layer.range;
            }
        }

        firewall.step();
    }

    severity
}

fn main() {
    let input = advent::download_input(2017, 13);

    println!("Step 1: {}", step1(&input));
}
