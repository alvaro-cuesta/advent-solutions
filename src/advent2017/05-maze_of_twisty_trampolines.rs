//! # [Day 5: A Maze of Twisty Trampolines, All Alike](http://adventofcode.com/2017/day/5)
//!
//! An urgent <span title="Later, on its turn, it sends you a sorcery.">
//! interrupt</span> arrives from the CPU: it's trapped in a maze of jump
//! instructions, and it would like assistance from any programs with spare
//! cycles to help find the exit.
//!

/// The message includes a list of the offsets for each jump. Jumps are
/// relative: `-1` moves to the previous instruction, and `2` skips the next
/// one. Start at the first instruction in the list. The goal is to follow
/// the jumps until one leads *outside* the list.
pub fn parse_input(input: &str) -> Vec<isize> {
    input
        .split_terminator('\n')
        .map(|x| x.parse::<isize>().expect("Unexpected non-integer jump"))
        .collect::<Vec<_>>()
}

/// In addition, these instructions are a little strange; after each jump,
/// the offset of that instruction increases by `1`. So, if you come across
/// an offset of `3`, you would move three instructions forward, but change
/// it to a `4` for the next time it is encountered.
///
/// For example, consider the following list of jump offsets:
///
/// ```text
/// 0
/// 3
/// 0
/// 1
/// -3
/// ```
///
/// Positive jumps ("forward") move downward; negative jumps move upward.
/// For legibility in this example, these offset values will be written all
/// on one line, with the current instruction marked in parentheses. The
/// following steps would be taken before an exit is found:
///
/// -   `(0) 3  0  1  -3 ` - *before* we have taken any steps.
/// -   `(1) 3  0  1  -3 ` - jump with offset `0` (that is, don't jump at
///     all). Fortunately, the instruction is then incremented to `1`.
/// -   ` 2 (3) 0  1  -3 ` - step forward because of the instruction we just
///     modified. The first instruction is incremented again, now to `2`.
/// -   ` 2  4  0  1 (-3)` - jump all the way to the end; leave a `4`
///     behind.
/// -   ` 2 (4) 0  1  -2 ` - go back to where we just were; increment `-3`
///     to `-2`.
/// -   ` 2  5  0  1  -2 ` - jump `4` steps forward, escaping the maze.
///
/// In this example, the exit is reached in `5` steps.
///
/// ```
/// # use advent_solutions::advent2017::day05::{ parse_input, part1 };
/// # let input = parse_input("0
/// # 3
/// # 0
/// # 1
/// # -3
/// # ");
/// assert_eq!(part1(input), 5);
/// ```
///
/// *How many steps* does it take to reach the exit?
pub fn part1(jumps: Vec<isize>) -> usize {
    count_steps(jumps, |ip| ip + 1 )
}

/// Now, the jumps are even stranger: after each jump, if the offset was
/// *three or more*, instead *decrease* it by `1`. Otherwise, increase it by
/// `1` as before.
///
/// Using this rule with the above example, the process now takes `10`
/// steps, and the offset values after finding the exit are left as
/// `2 3 2 3 -1`.
///
/// ```
/// # use advent_solutions::advent2017::day05::{ parse_input, part2 };
/// # let input = parse_input("0
/// # 3
/// # 0
/// # 1
/// # -3
/// # ");
/// assert_eq!(part2(input), 10);
/// ```
///
/// *How many steps* does it now take to reach the exit?
pub fn part2(jumps: Vec<isize>) -> usize {
    count_steps(jumps, |ip| if ip >= 3 { ip - 1 } else { ip + 1 } )
}

/// Counts the steps required to exit the maze, given a instruction mutation
/// function `mut_fn`.
pub fn count_steps<F: Fn(isize) -> isize>(mut memory: Vec<isize>, mut_fn: F) -> usize {
    let mut ip = 0;

    let length = memory.len();

    ::itertools::repeat_call(|| {
        let old_ip = ip;
        ip = (ip as isize + memory[ip]) as usize;
        memory[old_ip] = mut_fn(memory[old_ip]);
        ip
    })
    .take_while(|&ip| ip < length)
    .count()
    + 1
}

pub fn main(download: &::Download) {
    let jumps = parse_input(&download.input(2017, 5));

    println!("Part 1: {}", part1(jumps.clone()));
    println!("Part 2: {}", part2(jumps.clone()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_input() {
        let jumps = super::parse_input(include_str!("../../test_inputs/2017/05"));

        assert_eq!(super::part1(jumps.clone()), 360603);
        assert_eq!(super::part2(jumps.clone()), 25347697);
    }
}
