use crate::digraph::Digraph;

pub struct DirectedDFS {
    graph: Digraph,
    sources: Vec<usize>,
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
}

impl DirectedDFS {
    pub fn new(graph: Digraph, sources: Vec<usize>) -> Self {
        Self {
            sources,
            marked: vec![false; graph.vertex()],
            edge_to: vec![None; graph.vertex()],
            graph,
        }
    }
}
