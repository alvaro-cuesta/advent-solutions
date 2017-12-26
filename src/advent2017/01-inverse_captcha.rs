//! # [Day 1: Inverse Captcha](http://adventofcode.com/2017/day/1)
//!
//! The night before Christmas, one of Santa's Elves calls you in a panic.
//! "The printer's broken! We can't print the *Naughty or Nice List*!" By
//! the time you make it to <span title="Floor 17: cafeteria, printing
//! department, and experimental organic digitization equipment.">sub-basement
//! 17</span>, there are only a few minutes until midnight. "We have a big
//! problem," she says; "there must be almost *fifty* bugs in this system,
//! but nothing else can print The List. Stand in this square, quick!
//! There's no time to explain; if you can convince them to pay you in
//! **stars**, you'll be able to--" She pulls a lever and the world goes
//! blurry.
//!
//! When your eyes can focus again, everything seems a lot more pixelated
//! than before. She must have sent you inside the computer! You check the
//! system clock: *25 milliseconds* until midnight. With that much time, you
//! should be able to collect all **fifty stars** by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on
//! each <s>day</s> millisecond in the advent calendar; the second puzzle is
//! unlocked when you complete the first. Each puzzle grants **one star**.
//! Good luck!
//!
//! You're standing in a room with "digitization quarantine" written in LEDs
//! along one wall. The only door is locked, but it includes a small
//! interface. "Restricted Area - Strictly No Digitized Users Allowed."

/// Finds the sum of all digits that match the digit offset by `offset` in the
/// list.
///
/// The list is circular, so the digit after the last digit is the first digit
/// in the list.
pub fn count_matching(input: &str, offset: usize) -> u32 {
    input.chars()
        .zip(input.chars().cycle().skip(offset))
        .filter_map(|(a, b)| if a == b {
                Some(a.to_digit(10).expect("Unexpected non-digit in string"))
            } else {
                None
            }
        )
        .sum::<u32>()
}

/// It goes on to explain that you may only leave by solving a [captcha] to
/// prove you're *not* a human. Apparently, you only get one millisecond to
/// solve the captcha: too fast for a normal human, but it feels like hours
/// to you.
///
/// The captcha requires you to review a sequence of digits (your puzzle
/// input) and find the *sum* of all digits that match the *next* digit in
/// the list. The list is circular, so the digit after the last digit is the
/// *first* digit in the list.
///
/// For example:
///
/// -   `1122` produces a sum of `3` (`1` + `2`) because the first digit
///     (`1`) matches the second digit and the third digit (`2`) matches the
///     fourth digit.
///
///     ```
///     # use advent_solutions::advent2017::day01::part1;
///     assert_eq!(part1("1122"), 3);
///     ```
///
/// -   `1111` produces `4` because each digit (all `1`) matches the next.
///
///     ```
///     # use advent_solutions::advent2017::day01::part1;
///     assert_eq!(part1("1111"), 4);
///     ```
///
/// -   `1234` produces `0` because no digit matches the next.
///
///     ```
///     # use advent_solutions::advent2017::day01::part1;
///     assert_eq!(part1("1234"), 0);
///     ```
///
/// -   `91212129` produces `9` because the only digit that matches the next
///     one is the last digit, `9`.
///
///     ```
///     # use advent_solutions::advent2017::day01::part1;
///     assert_eq!(part1("91212129"), 9);
///     ```
///
/// *What is the solution* to your captcha?
///
///   [captcha]: https://en.wikipedia.org/wiki/CAPTCHA
pub fn part1(input: &str) -> u32 {
    count_matching(input, 1)
}

/// You notice a progress bar that jumps to 50% completion. Apparently, the
/// door isn't yet satisfied, but it did emit a **star** as encouragement. The
/// instructions change:
///
/// Now, instead of considering the *next* digit, it wants you to consider
/// the digit *halfway around* the circular list. That is, if your list
/// contains `10` items, only include a digit in your sum if the digit
/// `10/2 = 5` steps forward matches it. Fortunately, your list has an even
/// number of elements.
///
/// For example:
///
/// -   `1212` produces `6`: the list contains `4` items, and all four
///     digits match the digit `2` items ahead.
///
///     ```
///     # use advent_solutions::advent2017::day01::part2;
///     assert_eq!(part2("1212"), 6);
///     ```
///
/// -   `1221` produces `0`, because every comparison is between a `1` and a
///     `2`.
///
///     ```
///     # use advent_solutions::advent2017::day01::part2;
///     assert_eq!(part2("1221"), 0);
///     ```
///
/// -   `123425` produces `4`, because both `2`s match each other, but no
///     other digit has a match.
///
///     ```
///     # use advent_solutions::advent2017::day01::part2;
///     assert_eq!(part2("123425"), 4);
///     ```
///
/// -   `123123` produces `12`.
///
///     ```
///     # use advent_solutions::advent2017::day01::part2;
///     assert_eq!(part2("123123"), 12);
///     ```
///
/// -   `12131415` produces `4`.
///
///     ```
///     # use advent_solutions::advent2017::day01::part2;
///     assert_eq!(part2("12131415"), 4);
///     ```
///
/// *What is the solution* to your new captcha?
pub fn part2(input: &str) -> u32 {
    count_matching(input, input.len() / 2)
}

pub fn main() {
    let input = ::download::single_input(2017, 1);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_input() {
        let mut input = include_str!("../../test_inputs/2017/01");
        input = &input[..input.len() - 1];

        assert_eq!(super::part1(&input), 1341);
        assert_eq!(super::part2(&input), 1348);
    }
}
