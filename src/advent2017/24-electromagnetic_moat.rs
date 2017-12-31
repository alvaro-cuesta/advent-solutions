use petgraph::{ IntoWeightedEdge };
use petgraph::graph::{ UnGraph, NodeIndex, EdgeIndex };
use petgraph::visit::EdgeRef;
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
}

impl IntoWeightedEdge<usize> for Component {
    type NodeId = usize;

    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, usize) {
        (self.0, self.1, self.0 + self.1)
    }
}

impl<'a> IntoWeightedEdge<usize> for &'a Component {
    type NodeId = usize;

    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, usize) {
        (self.0, self.1, self.0 + self.1)
    }
}

type ComponentGraph = UnGraph<usize, usize, usize>;

fn dfs_step(
    path: &mut Vec<EdgeIndex<usize>>,
    last_index: NodeIndex<usize>,
    graph: &ComponentGraph,
    cmp_f: &Fn((usize, usize), (usize, usize)) -> bool,
) -> (usize, usize)
{
    let edges = graph.edges(last_index)
        .filter(|e| !path.contains(&e.id()))
        .collect::<Vec<_>>();

    if edges.len() == 0 {
        let strength = path.iter()
            .map(|&ei| graph.edge_weight(ei).unwrap())
            .sum::<usize>();

        return (path.len(), strength);
    }

    let mut max_len = 0;
    let mut max_strength = 0;

    for edge in edges {
        path.push(edge.id());

        let new_index = if edge.source() == last_index {
            edge.target()
        } else {
            edge.source()
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
    graph: &ComponentGraph,
    cmp_f: &Fn((usize, usize), (usize, usize)) -> bool,
) -> usize
{
    dfs_step(&mut vec![], 0.into(), graph, cmp_f).1
}

pub fn part1(graph: &ComponentGraph) -> usize {
    dfs(
        graph,
        &|(_, max_strength), (_, new_strength)| new_strength > max_strength
    )
}

pub fn part2(graph: &ComponentGraph) -> usize {
    dfs(
        graph,
        &|(max_len, max_strength), (new_len, new_strength)|
            new_len > max_len || (new_len == max_len && new_strength > max_strength)
    )
}

pub fn parse_input(input: &str) -> ComponentGraph {
    let edges = Component::list_from_bytes(input.as_bytes())
        .to_full_result()
        .expect("Error parsing components");

    UnGraph::from_edges(&edges)
}

test_day!("24", 1868, 1841);
