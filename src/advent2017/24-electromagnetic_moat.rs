use itertools::Itertools;
use ::parse::unsigned_number;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Component(usize, usize);

impl Component {
    named!{ from_bytes (&[u8]) -> Component,
        do_parse!(
            a: unsigned_number >>
            char!('/') >>
            b: unsigned_number >>

            (Component(a, b))
        )
    }

    named!{ list_from_bytes (&[u8]) -> Vec<Component>,
        lines!(Component::from_bytes)
    }

    fn strength(self) -> usize {
        self.0 + self.1
    }
}

fn dfs_step(
    path: &mut Vec<usize>,
    last_index: usize,
    graph: &Vec<Component>,
    cmp_f: &Fn((usize, usize), (usize, usize)) -> bool,
) -> (usize, usize)
{
    let edges = graph.iter()
        .positions(|&Component(a, b)| last_index == a || last_index == b)
        .filter(|e| !path.contains(e))
        .collect::<Vec<_>>();

    if edges.len() == 0 {
        let strength = path.iter()
            .map(|&ei| graph[ei].strength())
            .sum::<usize>();

        return (path.len(), strength);
    }

    let mut max_len = 0;
    let mut max_strength = 0;

    for edge in edges {
        path.push(edge);

        let new_index = if graph[edge].0 == last_index {
            graph[edge].1
        } else {
            graph[edge].0
        };

        let (new_len, new_strength) = dfs_step(path, new_index, graph, cmp_f);

        if cmp_f((max_len, max_strength), (new_len, new_strength)) {
            max_len = new_len;
            max_strength = new_strength;
        }

        path.pop();
    }

    (max_len, max_strength)
}

fn dfs(
    graph: &Vec<Component>,
    cmp_f: &Fn((usize, usize), (usize, usize)) -> bool,
) -> usize
{
    dfs_step(&mut vec![], 0, graph, cmp_f).1
}

pub fn part1(graph: &Vec<Component>) -> usize {
    dfs(
        graph,
        &|(_, max_strength), (_, new_strength)| new_strength > max_strength
    )
}

pub fn part2(graph: &Vec<Component>) -> usize {
    dfs(
        graph,
        &|(max_len, max_strength), (new_len, new_strength)|
            new_len > max_len || (new_len == max_len && new_strength > max_strength)
    )
}

pub fn parse_input(input: &str) -> Vec<Component> {
    Component::list_from_bytes(input.as_bytes())
        .to_full_result()
        .expect("Error parsing components")
}

test_day!("24", 1868, 1841);
