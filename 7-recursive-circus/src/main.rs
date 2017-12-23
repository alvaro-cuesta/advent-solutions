extern crate advent;
#[macro_use] extern crate nom;
use nom::{ alpha, digit };
use std::str::FromStr;
use std::collections::{ HashMap, HashSet };

#[derive(Clone, PartialEq, Eq, Debug)]
struct Node<'a> {
    name: &'a str,
    weight: usize,
    children: Vec<&'a str>,
}

impl<'a> Node<'a> {
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

named!(parse_name<&[u8], &str>, map_res!(alpha, std::str::from_utf8));

named!(parse_node<&[u8], Node>,
  do_parse!(
    name: parse_name >>

    weight: map_res!(
        map_res!(
            delimited!(tag!(" ("), digit, tag!(")")),
            std::str::from_utf8
        ),
        FromStr::from_str
    ) >>

    children: map!(
        opt!(do_parse!(
            tag!(" -> ") >>
            children: separated_nonempty_list_complete!(tag!(", "), parse_name) >>
            (children)
        )),
        |x| x.unwrap_or_else(|| Vec::new())
    )>>

    tag!("\n") >>

    (Node { name, weight, children })
  )
);

named!(parse_nodes<&[u8], Vec<Node>>, many0!(parse_node));

fn main() {
    let input = advent::download_input(2017, 7);

    let nodes = parse_nodes(input.as_bytes())
        .to_full_result()
        .expect("Error parsing nodes");

    let mut is_children = HashSet::new();

    for node in &nodes {
        for &child_name in &node.children {
            is_children.insert(child_name);
        }
    }

    let mut roots = nodes.iter()
        .filter(|x| !is_children.contains(x.name));

    let step1 = roots.next().expect("No roots found");
    assert!(roots.next() == None, "Found more than one root");

    println!("Step 1: {}", step1.name);

    let mut by_name = HashMap::new();

    for node in &nodes {
        by_name.insert(node.name, node);
    }

    let by_name_ref = &by_name;

    let imbalance_origins = nodes.iter()
        .filter(|node| !node.is_balanced(by_name_ref)
            && node.children.iter()
                .all(|child_name| by_name_ref[child_name].is_balanced(by_name_ref)))
        .collect::<Vec<_>>();

    assert!(imbalance_origins.len() == 1, "Found zero or more than one imbalance origin");

    let imbalanced_children = imbalance_origins[0]
        .children.iter()
        .map(|child_name| {
            let node = by_name_ref[child_name];
            let weight = node.tree_weight(by_name_ref);

            (node, weight)
        })
        .collect::<Vec<_>>();

    if let Some((
        &(min_node, min_weight),
        Some(&(max_node, max_weight))
    )) = advent::min_and_max_by_key(&imbalanced_children, |x| x.1)
    {
        let min_children = imbalanced_children.iter()
            .filter(|&&(_, weight)| weight == min_weight)
            .collect::<Vec<_>>();

        let max_children = imbalanced_children.iter()
            .filter(|&&(_, weight)| weight == max_weight)
            .collect::<Vec<_>>();

        let result = if min_children.len() == 1 {
            min_node.weight + max_weight - min_weight
        } else if max_children.len() == 1 {
            max_node.weight + min_weight - max_weight
        } else {
            panic!("More than one min/max");
        };

        println!("Step 2: {}", result);
    } else {
        panic!("Could not find min and max imbalanced children");
    }
}
