use crate::digraph::Digraph;

pub struct DirectedDFS<'a> {
    graph: &'a Digraph,
    sources: Vec<usize>,
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
}

impl<'a> DirectedDFS<'a> {
    pub fn new(graph: &'a Digraph, sources: Vec<usize>) -> Self {
        Self {
            sources,
            marked: vec![false; graph.vertex()],
            edge_to: vec![None; graph.vertex()],
            graph,
        }
    }
}
