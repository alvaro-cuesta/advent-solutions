//! # [Day 9: Stream Processing](http://adventofcode.com/2017/day/9)
//!
//! A large stream blocks your path. According to the locals, it's not safe
//! to <span title="&quot;Don't cross the streams!&quot;, they yell, even though
//! there's only one. They seem to think they're hilarious.">cross the
//! stream</span> at the moment because it's full of *garbage*. You look
//! down at the stream; rather than water, you discover that it's a *stream
//! of characters*.

use nom::anychar;

/// You sit for a while and record part of the stream (your puzzle input).
/// The characters represent *groups* - sequences that begin with `{` and
/// end with `}`. Within a group, there are zero or more other things,
/// separated by commas: either another *group* or *garbage*. Since groups
/// can contain other groups, a `}` only closes the *most-recently-opened
/// unclosed group* - that is, they are nestable. Your puzzle input
/// represents a single, large group which itself contains many smaller
/// ones.
///
/// Sometimes, instead of a group, you will find *garbage*. Garbage begins
/// with `<` and ends with `>`. Between those angle brackets, almost any
/// character can appear, including `{` and `}`. *Within* garbage, `<` has
/// no special meaning.
///
/// In a futile attempt to clean up the garbage, some program has *canceled*
/// some of the characters within it using `!`: inside garbage, *any*
/// character that comes after `!` should be *ignored*, including `<`, `>`,
/// and even another `!`.
///
/// You don't see any characters that deviate from these rules. Outside
/// garbage, you only find well-formed groups, and garbage always terminates
/// according to the rules above.
///
/// Here are some self-contained pieces of garbage:
///
/// -   `<>`, empty garbage.
/// -   `<random characters>`, garbage containing random characters.
/// -   `<<<<>`, because the extra `<` are ignored.
/// -   `<{!>}>`, because the first `>` is canceled.
/// -   `<!!>`, because the second `!` is canceled, allowing the `>` to
///     terminate the garbage.
/// -   `<!!!>>`, because the second `!` and the first `>` are canceled.
/// -   `<{o"i!a,<{i<a>`, which ends at the first `>`.
///
/// Here are some examples of whole streams and the number of groups they
/// contain:
///
/// -   `{}`, `1` group.
/// -   `{{{}}}`, `3` groups.
/// -   `{{},{}}`, also `3` groups.
/// -   `{{{},{},{{}}}}`, `6` groups.
/// -   `{<{},{},{{}}>}`, `1` group (which itself contains garbage).
/// -   `{<a>,<a>,<a>,<a>}`, `1` group.
/// -   `{{<a>},{<a>},{<a>},{<a>}}`, `5` groups.
/// -   `{{<!>},{<!>},{<!>},{<a>}}`, `2` groups (since all but the last `>`
///     are canceled).
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Node {
    Group(Vec<Node>),
    Garbage(usize),
}

use self::Node::*;

named!{ parse_garbage_char (&[u8]) -> usize,
    alt!(
        do_parse!(char!('!') >> anychar >> (0))
        |
        //do_parse!(not!(char!('>')) >> (1))
        do_parse!(verify!(anychar, |c| c != '>') >> (1))
    )
}

impl Node {
    pub fn score(&self, level: usize) -> usize {
        match *self {
            Group(ref children) =>
                level + children.iter()
                    .map(|node| node.score(level + 1))
                    .sum::<usize>(),
            Garbage(_) => 0,
        }
    }

    pub fn count_garbage(&self) -> usize {
        match *self {
            Group(ref children) => children.iter()
                .map(|node| node.count_garbage())
                .sum::<usize>(),
            Garbage(length) => length,
        }
    }

    named!{ group_from_bytes (&[u8]) -> Node,
        do_parse!(
            char!('{') >>
            children: separated_list!(char!(','), Node::from_bytes) >>
            char!('}') >>

            (Node::Group(children))
        )
    }

    named!{ garbage_from_bytes (&[u8]) -> Node,
        delimited!(
            char!('<'),
            map!(
                many0!(parse_garbage_char),
                |vec| Node::Garbage(vec.iter().sum::<usize>())
            ),
            char!('>')
        )
    }

    named!{ pub from_bytes (&[u8]) -> Node,
        alt!(call!(Node::group_from_bytes) | call!(Node::garbage_from_bytes))
    }
}

/// Your goal is to find the total score for all groups in your input. Each
/// group is assigned a *score* which is one more than the score of the
/// group that immediately contains it. (The outermost group gets a score of
/// `1`.)
///
/// -   `{}`, score of `1`.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part1 };
///     let root = Node::from_bytes(b"{}")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part1(&root), 1);
///     ```
///
/// -   `{{{}}}`, score of `1 + 2 + 3 = 6`.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part1 };
///     let root = Node::from_bytes(b"{{{}}}")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part1(&root), 6);
///     ```
///
/// -   `{{},{}}`, score of `1 + 2 + 2 = 5`.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part1 };
///     let root = Node::from_bytes(b"{{},{}}")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part1(&root), 5);
///     ```
///
/// -   `{{{},{},{{}}}}`, score of `1 + 2 + 3 + 3 + 3 + 4 = 16`.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part1 };
///     let root = Node::from_bytes(b"{{{},{},{{}}}}")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part1(&root), 16);
///     ```
///
/// -   `{<a>,<a>,<a>,<a>}`, score of `1`.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part1 };
///     let root = Node::from_bytes(b"{<a>,<a>,<a>,<a>}")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part1(&root), 1);
///     ```
///
/// -   `{{<ab>},{<ab>},{<ab>},{<ab>}}`, score of `1 + 2 + 2 + 2 + 2 = 9`.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part1 };
///     let root = Node::from_bytes(b"{{<ab>},{<ab>},{<ab>},{<ab>}}")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part1(&root), 9);
///     ```
///
/// -   `{{<!!>},{<!!>},{<!!>},{<!!>}}`, score of `1 + 2 + 2 + 2 + 2 = 9`.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part1 };
///     let root = Node::from_bytes(b"{{<!!>},{<!!>},{<!!>},{<!!>}}")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part1(&root), 9);
///     ```
///
/// -   `{{<a!>},{<a!>},{<a!>},{<ab>}}`, score of `1 + 2 = 3`.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part1 };
///     let root = Node::from_bytes(b"{{<a!>},{<a!>},{<a!>},{<ab>}}")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part1(&root), 3);
///     ```
///
/// *What is the total score* for all groups in your input?
pub fn part1(root: &Node) -> usize {
    if let &Node::Group(_) = root {
        root.score(1)
    } else {
        panic!("Unexpected garbage root node")
    }
}

/// Now, you're ready to remove the garbage.
///
/// To prove you've removed it, you need to count all of the characters
/// within the garbage. The leading and trailing `<` and `>` don't count,
/// nor do any canceled characters or the `!` doing the canceling.
///
/// -   `<>`, `0` characters.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part2 };
///     let root = Node::from_bytes(b"<>")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part2(&root), 0);
///     ```
///
/// -   `<random characters>`, `17` characters.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part2 };
///     let root = Node::from_bytes(b"<random characters>")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part2(&root), 17);
///     ```
///
/// -   `<<<<>`, `3` characters.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part2 };
///     let root = Node::from_bytes(b"<<<<>")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part2(&root), 3);
///     ```
///
/// -   `<{!>}>`, `2` characters.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part2 };
///     let root = Node::from_bytes(b"<{!>}>")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part2(&root), 2);
///     ```
///
/// -   `<!!>`, `0` characters.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part2 };
///     let root = Node::from_bytes(b"<!!>")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part2(&root), 0);
///     ```
///
/// -   `<!!!>>`, `0` characters.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part2 };
///     let root = Node::from_bytes(b"<!!!>>")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part2(&root), 0);
///     ```
///
/// -   `<{o"i!a,<{i<a>`, `10` characters.
///
///     ```
///     # use advent_solutions::advent2017::day09::{ Node, part2 };
///     let root = Node::from_bytes(b"<{o\"i!a,<{i<a>")
///         .to_result()
///         .unwrap();
///
///     assert_eq!(part2(&root), 10);
///     ```
///
/// *How many non-canceled characters are within the garbage* in your puzzle
/// input?
pub fn part2(root: &Node) -> usize {
    root.count_garbage()
}

pub fn main(download: &::Download) {
    let input = download.input(2017, 9);

    let root = Node::from_bytes(input.as_bytes())
        .to_full_result()
        .expect("Error parsing stream");

    println!("Part 1: {}", part1(&root));
    println!("Part 2: {}", part2(&root));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_input() {
        let input = include_str!("../../test_inputs/2017/09");

        let root = super::Node::from_bytes(input.as_bytes())
            .to_full_result()
            .expect("Error parsing stream");

        assert_eq!(super::part1(&root), 14204);
        assert_eq!(super::part2(&root), 6622);
    }
}
