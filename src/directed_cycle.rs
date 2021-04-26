use crate::digraph::Digraph;

pub struct DirectedCycle<'a> {
    graph: &'a Digraph,
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    cycle: Vec<usize>,
}

impl<'a> DirectedCycle<'a> {
    fn new(graph: &'a Digraph) -> Self {
        let mut dc = Self {
            graph,
            marked: vec![false; graph.vertex()],
            edge_to: vec![None; graph.vertex()],
            cycle: vec![],
        };
        for v in 0..graph.vertex() {
            if dc.marked[v] {
                continue;
            }
            dc.dfs(v);
        }
        dc
    }

    fn dfs(&mut self, v: usize) {}
}
