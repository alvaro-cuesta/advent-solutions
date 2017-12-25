//! # [Day 4: High-Entropy Passphrases](http://adventofcode.com/2017/day/4)
//!
//! A new system policy has been put in place that requires all accounts to
//! use a *passphrase* instead of simply a pass*word*. A passphrase consists
//! of a series of words (lowercase letters) separated by spaces.

/// To ensure security, a valid passphrase must contain no duplicate words.
///
/// For example:
///
/// -   `aa bb cc dd ee` is valid.
///
///     ```
///     # use advent_solutions::day4::is_valid_part1;
///     assert_eq!(is_valid_part1("aa bb cc dd ee"), true);
///     ```
///
/// -   `aa bb cc dd aa` is not valid - the word `aa` appears more than
///     once.
///
///     ```
///     # use advent_solutions::day4::is_valid_part1;
///     assert_eq!(is_valid_part1("aa bb cc dd aa"), false);
///     ```
///
/// -   `aa bb cc dd aaa` is valid - `aa` and `aaa` count as different
///     words.
///
///     ```
///     # use advent_solutions::day4::is_valid_part1;
///     assert_eq!(is_valid_part1("aa bb cc dd aaa"), true);
///     ```
pub fn is_valid_part1(passphrase: &str) -> bool {
    passphrase.split(' ')
        .all(|word| passphrase.split(' ')
            .filter(|w| &word == w)
            .count() == 1
        )
}

/// The system's full passphrase list is available as your puzzle input.
/// *How many passphrases are valid?*
pub fn part1(input: &str) -> usize {
    input.lines()
        .filter(|passphrase| is_valid_part1(passphrase))
        .count()
}

/// For added security, <span title="Because as everyone knows, the number of
/// rules is proportional to the level of security.">yet another system policy
/// </span> has been put in place. Now, a valid passphrase must contain no two
/// words that are anagrams of each other - that is, a passphrase is invalid
/// if any word's letters can be rearranged to form any other word in the
/// passphrase.
///
/// For example:
///
/// -  `abcde fghij` is a valid passphrase.
///
///   ```
///   # use advent_solutions::day4::is_valid_part2;
///   assert_eq!(is_valid_part2("abcde fghij"), true);
///   ```
///
/// - `abcde xyz ecdab` is not valid - the letters from the third word can
///   be rearranged to form the first word.
///
///   ```
///   # use advent_solutions::day4::is_valid_part2;
///   assert_eq!(is_valid_part2("abcde xyz ecdab"), false);
///   ```
///
/// - `a ab abc abd abf abj` is a valid passphrase, because *all* letters
///   need to be used when forming another word.
///
///   ```
///   # use advent_solutions::day4::is_valid_part2;
///   assert_eq!(is_valid_part2("a ab abc abd abf abj"), true);
///   ```
///
/// - `iiii oiii ooii oooi oooo` is valid.
///
///   ```
///   # use advent_solutions::day4::is_valid_part2;
///   assert_eq!(is_valid_part2("iiii oiii ooii oooi oooo"), true);
///   ```
///
/// - `oiii ioii iioi iiio` is not valid - any of these words can be
///   rearranged to form any other word.
///
///   ```
///   # use advent_solutions::day4::is_valid_part2;
///   assert_eq!(is_valid_part2("oiii ioii iioi iiio"), false);
///   ```
pub fn is_valid_part2(passphrase: &str) -> bool {
    passphrase.split(' ')
        .all(|a| {
            let mut a_chars = a.chars().collect::<Vec<_>>();
            a_chars.sort();

            passphrase.split(' ')
                .filter(|b| {
                    let mut b_chars = b.chars().collect::<Vec<_>>();
                    b_chars.sort();

                    a_chars == b_chars
                })
                .count() == 1
        })
}

/// Under this new system policy, *how many passphrases are valid?*
pub fn part2(input: &str) -> usize {
    input.lines()
        .filter(|passphrase| is_valid_part2(passphrase))
        .count()
}

pub fn main() {
    let input = ::advent::download_input(2017, 4);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
