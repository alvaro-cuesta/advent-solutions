//! # [Day 7: Recursive Circus](http://adventofcode.com/2017/day/7)
//!
//! Wandering further through the circuits of the computer, you come upon a
//! tower of <span title="Turtles, all the way down.">programs</span> that
//! have gotten themselves into a bit of trouble. A recursive algorithm has
//! gotten out of hand, and now they're balanced precariously in a large
//! tower.
//!
//! One program at the bottom supports the entire tower. It's holding a
//! large disc, and on the disc are balanced several more sub-towers. At the
//! bottom of these sub-towers, standing on the bottom disc, are other
//! programs, each holding *their* own disc, and so on. At the very tops of
//! these sub-sub-sub-...-towers, many programs stand simply keeping the
//! disc below them balanced but with no disc of their own.

use ::std::collections::HashMap;
use ::nom::digit;
use ::parse::{ name as parse_name };

/// You offer to help, but first you need to understand the structure of
/// these towers. You ask each program to yell out their *name*, their
/// *weight*, and (if they're holding a disc) the *names of the programs
/// immediately above them* balancing on that disc. You write this
/// information down (your puzzle input). Unfortunately, in their panic,
/// they don't do this in an orderly fashion; by the time you're done,
/// you're not sure which program gave which information.
///
/// For example, if your list is the following:
///
/// ```text
/// pbga (66)
/// xhth (57)
/// ebii (61)
/// havc (66)
/// ktlj (57)
/// fwft (72) -> ktlj, cntj, xhth
/// qoyq (66)
/// padx (45) -> pbga, havc, qoyq
/// tknk (41) -> ugml, padx, fwft
/// jptl (61)
/// ugml (68) -> gyxo, ebii, jptl
/// gyxo (61)
/// cntj (57)
/// ```
///
/// ...then you would be able to recreate the structure of the towers that
/// looks like this:
///
/// ```text
///                 gyxo
///               /
///          ugml - ebii
///        /      \
///       |         jptl
///       |
///       |         pbga
///      /        /
/// tknk --- padx - havc
///      \        \
///       |         qoyq
///       |
///       |         ktlj
///        \      /
///          fwft - cntj
///               \
///                 xhth
/// ```
///
/// In this example, `tknk` is at the bottom of the tower (the *bottom
/// program*), and is holding up `ugml`, `padx`, and `fwft`. Those programs
/// are, in turn, holding up other programs; in this example, none of those
/// programs are holding up any other programs, and are all the tops of
/// their own towers. (The actual tower balancing in front of you is much
/// larger.)
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Node<'a> {
    name: &'a str,
    weight: usize,
    children: Vec<&'a str>,
}

impl<'a> Node<'a> {
    named!{ pub from_bytes(&[u8]) -> Node,
        do_parse!(
            name: parse_name >>

            weight: from_str_bytes!(delimited!(tag!(" ("), digit, tag!(")"))) >>

            children: map!(
                opt!(do_parse!(
                    tag!(" -> ") >>
                    children: separated_nonempty_list_complete!(tag!(", "), parse_name) >>
                    (children)
                )),
                |x| x.unwrap_or_else(|| Vec::new())
            ) >>

            (Node { name, weight, children })
        )
    }

    named!{ pub list_from_bytes(&[u8]) -> Vec<Node>,
        lines!(Node::from_bytes)
    }

    fn is_balanced(&'a self, nodes: &HashMap<&'a str, &'a Node>) -> bool {
        if self.children.len() < 2 { return true; }

        let first_weight = nodes[self.children[0]].tree_weight(nodes);

        self.children[1..].iter()
            .map(|child_name| nodes[child_name].tree_weight(nodes))
            .all(|weight| weight == first_weight)
    }

    fn tree_weight(&'a self, nodes: &HashMap<&'a str, &'a Node>) -> usize {
        self.weight
        + self.children.iter()
            .map(|child_name| nodes[child_name].tree_weight(nodes))
            .sum::<usize>()
    }
}

/// Before you're ready to help them, you need to make sure your information
/// is correct. *What is the name of the bottom program?*
///
/// ```
/// # use advent_solutions::advent2017::day07::{ Node, part1 };
/// let input = b"pbga (66)
/// xhth (57)
/// ebii (61)
/// havc (66)
/// ktlj (57)
/// fwft (72) -> ktlj, cntj, xhth
/// qoyq (66)
/// padx (45) -> pbga, havc, qoyq
/// tknk (41) -> ugml, padx, fwft
/// jptl (61)
/// ugml (68) -> gyxo, ebii, jptl
/// gyxo (61)
/// cntj (57)
/// ";
///
/// let graph = Node::list_from_bytes(input)
///     .to_result()
///     .unwrap();
///
/// assert_eq!(part1(&graph), "tknk");
/// ```
pub fn part1<'a, 'b>(nodes: &'a [Node<'b>]) -> &'b str {
    use std::collections::HashSet;

    let mut is_children = HashSet::new();

    for node in nodes {
        for &child_name in &node.children {
            is_children.insert(child_name);
        }
    }

    let mut roots = nodes.iter()
        .filter(|x| !is_children.contains(x.name));

    let root = roots.next().expect("No roots found");
    assert!(roots.next() == None, "Found more than one root");

    root.name
}

/// The programs explain the situation: they can't get down. Rather, they
/// *could* get down, if they weren't expending all of their energy trying
/// to keep the tower balanced. Apparently, one program has the *wrong
/// weight*, and until it's fixed, they're stuck here.
///
/// For any program holding a disc, each program standing on that disc forms
/// a sub-tower. Each of those sub-towers are supposed to be the same
/// weight, or the disc itself isn't balanced. The weight of a tower is the
/// sum of the weights of the programs in that tower.
///
/// In the example above, this means that for `ugml`'s disc to be balanced,
/// `gyxo`, `ebii`, and `jptl` must all have the same weight, and they do:
/// `61`.
///
/// However, for `tknk` to be balanced, each of the programs standing on its
/// disc *and all programs above it* must each match. This means that the
/// following sums must all be the same:
///
/// -   `ugml` + (`gyxo` + `ebii` + `jptl`) = 68 + (61 + 61 + 61) = 251
/// -   `padx` + (`pbga` + `havc` + `qoyq`) = 45 + (66 + 66 + 66) = 243
/// -   `fwft` + (`ktlj` + `cntj` + `xhth`) = 72 + (57 + 57 + 57) = 243
///
/// As you can see, `tknk`'s disc is unbalanced: `ugml`'s stack is heavier
/// than the other two. Even though the nodes above `ugml` are balanced,
/// `ugml` itself is too heavy: it needs to be `8` units lighter for its
/// stack to weigh `243` and keep the towers balanced. If this change were
/// made, its weight would be `60`.
///
/// ```
/// # use advent_solutions::advent2017::day07::{ Node, part2 };
/// let input = b"pbga (66)
/// xhth (57)
/// ebii (61)
/// havc (66)
/// ktlj (57)
/// fwft (72) -> ktlj, cntj, xhth
/// qoyq (66)
/// padx (45) -> pbga, havc, qoyq
/// tknk (41) -> ugml, padx, fwft
/// jptl (61)
/// ugml (68) -> gyxo, ebii, jptl
/// gyxo (61)
/// cntj (57)
/// ";
///
/// let graph = Node::list_from_bytes(input)
///     .to_result()
///     .unwrap();
///
/// assert_eq!(part2(&graph), 60);
/// ```
///
/// Given that exactly one program is the wrong weight, *what would its
/// weight need to be* to balance the entire tower?
pub fn part2(nodes: &[Node]) -> usize {
    let mut by_name = HashMap::new();

    for node in nodes {
        by_name.insert(node.name, node);
    }

    let imbalance_origins = nodes.iter()
        .filter(|node| !node.is_balanced(&by_name)
            && node.children.iter()
                .all(|child_name| by_name[child_name].is_balanced(&by_name)))
        .collect::<Vec<_>>();

    assert!(imbalance_origins.len() == 1, "Found zero or more than one imbalance origin");

    let imbalanced_children = imbalance_origins[0]
        .children.iter()
        .map(|child_name| {
            let node = by_name[child_name];
            let weight = node.tree_weight(&by_name);

            (node, weight)
        })
        .collect::<Vec<_>>();

    if let Some((
        &(min_node, min_weight),
        Some(&(max_node, max_weight))
    )) = ::iter::min_and_max_by_key(&imbalanced_children, |x| x.1)
    {
        let min_children = imbalanced_children.iter()
            .filter(|&&(_, weight)| weight == min_weight)
            .collect::<Vec<_>>();

        let max_children = imbalanced_children.iter()
            .filter(|&&(_, weight)| weight == max_weight)
            .collect::<Vec<_>>();

        if min_children.len() == 1 {
            min_node.weight + max_weight - min_weight
        } else if max_children.len() == 1 {
            max_node.weight + min_weight - max_weight
        } else {
            panic!("More than one min/max");
        }
    } else {
        panic!("Could not find min and max imbalanced children");
    }
}

pub fn main() {
    let input = ::download::input(2017, 7);

    let nodes = Node::list_from_bytes(input.as_bytes())
        .to_full_result()
        .expect("Error parsing nodes");

    println!("Part 1: {}", part1(&nodes));
    println!("Part 2: {}", part2(&nodes));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_input() {
        let input = include_str!("../../test_inputs/2017/07");

        let nodes = super::Node::list_from_bytes(input.as_bytes())
            .to_full_result()
            .expect("Error parsing nodes");

        assert_eq!(super::part1(&nodes), "mkxke");
        assert_eq!(super::part2(&nodes), 268);
    }
}
