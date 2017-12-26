//! # [Day 2: Corruption Checksum](http://adventofcode.com/2017/day/2)
//!
//! As you walk through the door, a glowing humanoid shape yells in your
//! direction. "You there! Your state appears to be idle. Come help us
//! repair the corruption in this spreadsheet - if we take another
//! millisecond, we'll have to display an hourglass cursor!"

/// The spreadsheet consists of rows of apparently-random numbers. To make
/// sure the recovery process is on the right track, they need you to
/// calculate the spreadsheet's *checksum*. For each row, determine the
/// difference between the largest value and the smallest value; the
/// checksum is the sum of all of these differences.
///
/// For example, given the following spreadsheet:
///
/// ```text
/// 5 1 9 5
/// 7 5 3
/// 2 4 6 8
/// ```
///
/// -   The first row's largest and smallest values are `9` and `1`, and
///     their difference is `8`.
/// -   The second row's largest and smallest values are `7` and `3`, and
///     their difference is `4`.
/// -   The third row's difference is `6`.
///
/// In this example, the spreadsheet's checksum would be `8 + 4 + 6 = 18`.
///
/// ```
/// # use advent_solutions::advent2017::day02::{ parse_input, part1 };
/// # let input = "5\t1\t9\t5
/// # 7\t5\t3
/// # 2\t4\t6\t8
/// # ";
/// assert_eq!(part1(&parse_input(input)), 18);
/// ```
///
/// *What is the checksum* for the spreadsheet in your puzzle input?
pub fn part1<'a, I, J>(lines: I) -> usize
    where I: IntoIterator<Item=J>,
          J: IntoIterator<Item=&'a usize>,
{
    lines.into_iter()
        .map(|line| match ::iter::min_and_max(line) {
            Some((min, Some(max))) => max - min,
            Some((_, None)) => 0,
            _ => panic!("Unexpected empty line"),
        })
        .sum::<usize>()
}

/// "Great work; looks like we're on the right track after all. Here's a
/// **star** for your effort." However, the program seems a little worried.
/// Can programs *be* worried?
///
/// "Based on what we're seeing, it looks like all the User wanted is some
/// information about the *evenly divisible values* in the spreadsheet.
/// Unfortunately, none of us are equipped for that kind of calculation -
/// most of us specialize in <span title="Bonus points if you solve this part
/// using only bitwise operations.">bitwise operations</span>."
///
/// It sounds like the goal is to find the only two numbers in each row
/// where one evenly divides the other - that is, where the result of the
/// division operation is a whole number. They would like you to find those
/// numbers on each line, divide them, and add up each line's result.
///
/// For example, given the following spreadsheet:
///
/// ```text
/// 5 9 2 8
/// 9 4 7 3
/// 3 8 6 5
/// ```
///
/// -   In the first row, the only two numbers that evenly divide are `8`
///     and `2`; the result of this division is `4`.
/// -   In the second row, the two numbers are `9` and `3`; the result is `3`.
/// -   In the third row, the result is `2`.
///
/// In this example, the sum of the results would be `4 + 3 + 2 = 9`.
///
/// ```
/// # use advent_solutions::advent2017::day02::{ parse_input, part2 };
/// # let input = "5\t9\t2\t8
/// # 9\t4\t7\t3
/// # 3\t8\t6\t5
/// # ";
/// assert_eq!(part2(&parse_input(input)), 9);
/// ```
///
/// What is the *sum of each row's result* in your puzzle input?
pub fn part2<'a, I, J>(lines: I) -> usize
    where I: IntoIterator<Item=J>,
          J: IntoIterator<Item=&'a usize> + Copy,
{
    lines.into_iter()
        .map(|line| {
            let mut divisible_pairs = line.into_iter()
                .enumerate()
                .filter_map(|(i, x)| line.into_iter()
                    .skip(i + 1)
                    .find(|&y| (x % y) == 0 || (y % x) == 0)
                    .map(|y| (x, y))
                );

            let (x, y) = divisible_pairs.next().expect("No divisible pair found");

            assert!(divisible_pairs.next().is_none(), "More than one divisible pairs");

            if x > y { x / y } else { y / x }
        })
        .sum::<usize>()
}

/// Parses input into a grid of numbers.
pub fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|line| line.split('\t')
            .map(|x| x.parse::<usize>().expect("Unexpected non-integer in spreadsheet"))
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>()
}

pub fn main() {
    let lines = parse_input(&::download::input(2017, 2));

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_input() {
        let input = super::parse_input(include_str!("../../test_inputs/2017/02"));

        assert_eq!(super::part1(&input), 34925);
        assert_eq!(super::part2(&input), 221);
    }
}
