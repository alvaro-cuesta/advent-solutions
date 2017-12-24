extern crate advent;
#[macro_use] extern crate nom;

use nom::anychar;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Node {
    Group(Vec<Node>),
    Garbage(usize),
}

impl Node {
    fn score(&self, level: usize) -> usize {
        use Node::*;

        match *self {
            Group(ref children) =>
                level + children.iter()
                    .map(|node| node.score(level + 1))
                    .sum::<usize>(),
            Garbage(_) => 0,
        }
    }

    fn garbage(&self) -> usize {
        use Node::*;

        match *self {
            Group(ref children) => children.iter()
                .map(|node| node.garbage())
                .sum::<usize>(),
            Garbage(length) => length,
        }
    }
}

named!{ parse_garbage_char (&[u8]) -> usize,
    alt!(
        do_parse!(tag!("!") >> anychar >> (0))
        |
        do_parse!(verify!(anychar, |c| c != '>') >> (1))
    )
}

named!{ parse_garbage (&[u8]) -> Node,
    do_parse!(
        tag!("<") >>
        length: map!(
            many0!(parse_garbage_char),
            |vec| vec.iter().sum::<usize>()
        ) >>
        tag!(">") >>
        (Node::Garbage(length))
    )
}

named!{ parse_group (&[u8]) -> Node,
    do_parse!(
        tag!("{") >>
        children: separated_list!(tag!(","), alt!(parse_group | parse_garbage)) >>
        tag!("}") >>
        (Node::Group(children))
    )
}

fn main() {
    let input = advent::download_input(2017, 9);

    let group = parse_group(input.as_bytes())
            .to_full_result()
            .expect("Error parsing stream");

    println!("Step 1: {}", group.score(1));
    println!("Step 2: {}", group.garbage());
}
