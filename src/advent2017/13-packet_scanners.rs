//! # [Day 13: Packet Scanners](http://adventofcode.com/2017/day/13)
//!
//! You need to cross a vast *firewall*. The firewall consists of several
//! layers, each with a *security scanner* that moves back and forth across
//! the layer. To succeed, you must not be detected by a scanner.

use std::str::FromStr;
use std::ops;

/// Within each layer, a security scanner moves back and forth within its
/// range. Each security scanner starts at the top and moves down until it
/// reaches the bottom, then moves up until it reaches the top, and repeats.
/// A security scanner takes *one picosecond* to move one step. Drawing
/// scanners as `S`, the first few picoseconds look like this:
///
/// ```text
/// Picosecond 0:
///  0   1   2   3   4   5   6
/// [S] [S] ... ... [S] ... [S]
/// [ ] [ ]         [ ]     [ ]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
///
/// Picosecond 1:
///  0   1   2   3   4   5   6
/// [ ] [ ] ... ... [ ] ... [ ]
/// [S] [S]         [S]     [S]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
///
/// Picosecond 2:
///  0   1   2   3   4   5   6
/// [ ] [S] ... ... [ ] ... [ ]
/// [ ] [ ]         [ ]     [ ]
/// [S]             [S]     [S]
///                 [ ]     [ ]
///
/// Picosecond 3:
///  0   1   2   3   4   5   6
/// [ ] [ ] ... ... [ ] ... [ ]
/// [S] [S]         [ ]     [ ]
/// [ ]             [ ]     [ ]
///                 [S]     [S]
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Layer {
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
    type Err = ::std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Layer {
            range: s.parse::<usize>()?,
            position: 0,
            forward: true,
        })
    }
}

#[cfg(test)]
#[test]
fn test_layers() {
    let input = "0: 3
1: 2
4: 4
6: 4
";
    let mut firewall = Firewall::new(&input);

    // Picosecond 0
    assert_eq!(firewall[0].unwrap().position, 0);
    assert_eq!(firewall[1].unwrap().position, 0);
    assert_eq!(firewall[4].unwrap().position, 0);
    assert_eq!(firewall[6].unwrap().position, 0);

    // Picosecond 1
    firewall.step();
    assert_eq!(firewall[0].unwrap().position, 1);
    assert_eq!(firewall[1].unwrap().position, 1);
    assert_eq!(firewall[4].unwrap().position, 1);
    assert_eq!(firewall[6].unwrap().position, 1);

    // Picosecond 2
    firewall.step();
    assert_eq!(firewall[0].unwrap().position, 2);
    assert_eq!(firewall[1].unwrap().position, 0);
    assert_eq!(firewall[4].unwrap().position, 2);
    assert_eq!(firewall[6].unwrap().position, 2);

    // Picosecond 3
    firewall.step();
    assert_eq!(firewall[0].unwrap().position, 1);
    assert_eq!(firewall[1].unwrap().position, 1);
    assert_eq!(firewall[4].unwrap().position, 3);
    assert_eq!(firewall[6].unwrap().position, 3);
}

/// By studying the firewall briefly, you are able to record (in your puzzle
/// input) the *depth* of each layer and the *range* of the scanning area
/// for the scanner within it, written as `depth: range`. Each layer has a
/// thickness of exactly `1`. A layer at depth `0` begins immediately inside
/// the firewall; a layer at depth `1` would start immediately after that.
///
/// For example, suppose you've recorded the following:
///
/// ```text
/// 0: 3
/// 1: 2
/// 4: 4
/// 6: 4
/// ```
///
/// This means that there is a layer immediately inside the firewall (with
/// range `3`), a second layer immediately after that (with range `2`), a
/// third layer which begins at depth `4` (with range `4`), and a fourth
/// layer which begins at depth 6 (also with range `4`). Visually, it might
/// look like this:
///
/// ```text
///  0   1   2   3   4   5   6
/// [ ] [ ] ... ... [ ] ... [ ]
/// [ ] [ ]         [ ]     [ ]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
/// ```
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Firewall(Vec<Option<Layer>>);

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

    pub fn step(&mut self) {
        for layer in &mut self.0 {
            layer.iter_mut().for_each(|x| x.step());
        }
    }
}

impl ops::Index<usize> for Firewall {
    type Output = Option<Layer>;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl ops::IndexMut<usize> for Firewall {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

#[cfg(test)]
#[test]
fn test_firewall() {
    let input = "0: 3
1: 2
4: 4
6: 4
";
    let firewall = Firewall::new(&input);

    assert_eq!(firewall.len(), 7);
    assert_eq!(firewall[0].unwrap().range, 3);
    assert_eq!(firewall[1].unwrap().range, 2);
    assert_eq!(firewall[2], None);
    assert_eq!(firewall[3], None);
    assert_eq!(firewall[4].unwrap().range, 4);
    assert_eq!(firewall[5], None);
    assert_eq!(firewall[6].unwrap().range, 4);
}

/// Your plan is to hitch a ride on a packet about to move through the
/// firewall. The packet will travel along the top of each layer, and it
/// moves at *one layer per picosecond*. Each picosecond, the packet moves
/// one layer forward (its first move takes it into layer 0), and then the
/// scanners move one step. If there is a scanner at the top of the layer
/// *as your packet enters it*, you are *caught*. (If a scanner moves into
/// the top of its layer while you are there, you are *not* caught: it
/// doesn't have time to notice you before you leave.) If you were to do
/// this in the configuration above, marking your current position with
/// parentheses, your passage through the firewall would look like this:
///
/// ```text
/// Initial state:
///  0   1   2   3   4   5   6
/// [S] [S] ... ... [S] ... [S]
/// [ ] [ ]         [ ]     [ ]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
///
/// Picosecond 0:
///  0   1   2   3   4   5   6
/// (S) [S] ... ... [S] ... [S]
/// [ ] [ ]         [ ]     [ ]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
///
///  0   1   2   3   4   5   6
/// ( ) [ ] ... ... [ ] ... [ ]
/// [S] [S]         [S]     [S]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
///
///
/// Picosecond 1:
///  0   1   2   3   4   5   6
/// [ ] ( ) ... ... [ ] ... [ ]
/// [S] [S]         [S]     [S]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
///
///  0   1   2   3   4   5   6
/// [ ] (S) ... ... [ ] ... [ ]
/// [ ] [ ]         [ ]     [ ]
/// [S]             [S]     [S]
///                 [ ]     [ ]
///
///
/// Picosecond 2:
///  0   1   2   3   4   5   6
/// [ ] [S] (.) ... [ ] ... [ ]
/// [ ] [ ]         [ ]     [ ]
/// [S]             [S]     [S]
///                 [ ]     [ ]
///
///  0   1   2   3   4   5   6
/// [ ] [ ] (.) ... [ ] ... [ ]
/// [S] [S]         [ ]     [ ]
/// [ ]             [ ]     [ ]
///                 [S]     [S]
///
///
/// Picosecond 3:
///  0   1   2   3   4   5   6
/// [ ] [ ] ... (.) [ ] ... [ ]
/// [S] [S]         [ ]     [ ]
/// [ ]             [ ]     [ ]
///                 [S]     [S]
///
///  0   1   2   3   4   5   6
/// [S] [S] ... (.) [ ] ... [ ]
/// [ ] [ ]         [ ]     [ ]
/// [ ]             [S]     [S]
///                 [ ]     [ ]
///
///
/// Picosecond 4:
///  0   1   2   3   4   5   6
/// [S] [S] ... ... ( ) ... [ ]
/// [ ] [ ]         [ ]     [ ]
/// [ ]             [S]     [S]
///                 [ ]     [ ]
///
///  0   1   2   3   4   5   6
/// [ ] [ ] ... ... ( ) ... [ ]
/// [S] [S]         [S]     [S]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
///
///
/// Picosecond 5:
///  0   1   2   3   4   5   6
/// [ ] [ ] ... ... [ ] (.) [ ]
/// [S] [S]         [S]     [S]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
///
///  0   1   2   3   4   5   6
/// [ ] [S] ... ... [S] (.) [S]
/// [ ] [ ]         [ ]     [ ]
/// [S]             [ ]     [ ]
///                 [ ]     [ ]
///
///
/// Picosecond 6:
///  0   1   2   3   4   5   6
/// [ ] [S] ... ... [S] ... (S)
/// [ ] [ ]         [ ]     [ ]
/// [S]             [ ]     [ ]
///                 [ ]     [ ]
///
///  0   1   2   3   4   5   6
/// [ ] [ ] ... ... [ ] ... ( )
/// [S] [S]         [S]     [S]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
/// ```
///
/// In this situation, you are *caught* in layers `0` and `6`, because your
/// packet entered the layer when its scanner was at the top when you
/// entered it. You are *not* caught in layer `1`, since the scanner moved
/// into the top of the layer once you were already there.
///
/// The *severity* of getting caught on a layer is equal to its *depth*
/// multiplied by its *range*. (Ignore layers in which you do not get
/// caught.) The severity of the whole trip is the sum of these values. In
/// the example above, the trip severity is `0*3 + 6*4 = 24`.
///
/// ```
/// # use advent_solutions::advent2017::day13::part1;
/// # let input = "0: 3
/// # 1: 2
/// # 4: 4
/// # 6: 4
/// # ";
/// assert_eq!(part1(&input), 24);
/// ```
///
/// Given the details of the firewall you've recorded, if you leave
/// immediately, *what is the severity of your whole trip*?
pub fn part1(input: &str) -> usize {
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

/// Now, you need to pass through the firewall without being caught - easier
/// said than done.
///
/// You can't control the <span title="Seriously, what network stack doesn't
/// let you adjust the speed of light?">speed of the packet</span>, but you
/// can *delay* it any number of picoseconds. For each picosecond you delay
/// the packet before beginning your trip, all security scanners move one step.
/// You're not in the firewall during this time; you don't enter layer `0`
/// until you stop delaying the packet.
///
/// In the example above, if you delay `10` picoseconds (picoseconds `0` -
/// `9`), you won't get caught:
///
/// ```text
/// State after delaying:
///  0   1   2   3   4   5   6
/// [ ] [S] ... ... [ ] ... [ ]
/// [ ] [ ]         [ ]     [ ]
/// [S]             [S]     [S]
///                 [ ]     [ ]
///
/// Picosecond 10:
///  0   1   2   3   4   5   6
/// ( ) [S] ... ... [ ] ... [ ]
/// [ ] [ ]         [ ]     [ ]
/// [S]             [S]     [S]
///                 [ ]     [ ]
///
///  0   1   2   3   4   5   6
/// ( ) [ ] ... ... [ ] ... [ ]
/// [S] [S]         [S]     [S]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
///
///
/// Picosecond 11:
///  0   1   2   3   4   5   6
/// [ ] ( ) ... ... [ ] ... [ ]
/// [S] [S]         [S]     [S]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
///
///  0   1   2   3   4   5   6
/// [S] (S) ... ... [S] ... [S]
/// [ ] [ ]         [ ]     [ ]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
///
///
/// Picosecond 12:
///  0   1   2   3   4   5   6
/// [S] [S] (.) ... [S] ... [S]
/// [ ] [ ]         [ ]     [ ]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
///
///  0   1   2   3   4   5   6
/// [ ] [ ] (.) ... [ ] ... [ ]
/// [S] [S]         [S]     [S]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
///
///
/// Picosecond 13:
///  0   1   2   3   4   5   6
/// [ ] [ ] ... (.) [ ] ... [ ]
/// [S] [S]         [S]     [S]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
///
///  0   1   2   3   4   5   6
/// [ ] [S] ... (.) [ ] ... [ ]
/// [ ] [ ]         [ ]     [ ]
/// [S]             [S]     [S]
///                 [ ]     [ ]
///
///
/// Picosecond 14:
///  0   1   2   3   4   5   6
/// [ ] [S] ... ... ( ) ... [ ]
/// [ ] [ ]         [ ]     [ ]
/// [S]             [S]     [S]
///                 [ ]     [ ]
///
///  0   1   2   3   4   5   6
/// [ ] [ ] ... ... ( ) ... [ ]
/// [S] [S]         [ ]     [ ]
/// [ ]             [ ]     [ ]
///                 [S]     [S]
///
///
/// Picosecond 15:
///  0   1   2   3   4   5   6
/// [ ] [ ] ... ... [ ] (.) [ ]
/// [S] [S]         [ ]     [ ]
/// [ ]             [ ]     [ ]
///                 [S]     [S]
///
///  0   1   2   3   4   5   6
/// [S] [S] ... ... [ ] (.) [ ]
/// [ ] [ ]         [ ]     [ ]
/// [ ]             [S]     [S]
///                 [ ]     [ ]
///
///
/// Picosecond 16:
///  0   1   2   3   4   5   6
/// [S] [S] ... ... [ ] ... ( )
/// [ ] [ ]         [ ]     [ ]
/// [ ]             [S]     [S]
///                 [ ]     [ ]
///
///  0   1   2   3   4   5   6
/// [ ] [ ] ... ... [ ] ... ( )
/// [S] [S]         [S]     [S]
/// [ ]             [ ]     [ ]
///                 [ ]     [ ]
/// ```
///
/// Because all smaller delays would get you caught, the fewest number of
/// picoseconds you would need to delay to get through safely is `10`.
///
/// ```
/// # use advent_solutions::advent2017::day13::part2;
/// # let input = "0: 3
/// # 1: 2
/// # 4: 4
/// # 6: 4
/// # ";
/// assert_eq!(part2(&input), 10);
/// ```
///
/// *What is the fewest number of picoseconds* that you need to delay the
/// packet to pass through the firewall without being caught?
pub fn part2(input: &str) -> usize {
    let firewall = Firewall::new(&input);

    for delay in 0.. {
        let caught = firewall.0.iter()
            .enumerate()
            .any(|(i, layer)| match *layer {
                Some(Layer { range, .. }) => (delay + i) % (range * 2 - 2) == 0,
                None => false,
            });

        if !caught {
            return delay;
        }
    }

    unreachable!();
}

pub fn main() {
    let input = ::download::input(2017, 13);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
