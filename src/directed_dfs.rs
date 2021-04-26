use crate::digraph::Digraph;

pub struct DirectedDFS<'a> {
    graph: &'a Digraph,
    sources: Vec<usize>,
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
}

impl<'a> DirectedDFS<'a> {
    pub fn new(graph: &'a Digraph, sources: Vec<usize>) -> Self {
        let mut dfs = Self {
            sources,
            marked: vec![false; graph.vertex()],
            edge_to: vec![None; graph.vertex()],
            graph,
        };
        for v in dfs.sources.clone() {
            dfs.dfs(v);
        }
        dfs
    }

    fn marked(&self, v: usize) -> bool {
        return self.marked[v];
    }

    fn dfs(&mut self, v: usize) {
        self.marked[v] = true;
        for w in self.graph.adj(v) {
            if self.marked(w) {
                continue;
            }
            self.edge_to[w] = Some(v);
            self.dfs(w);
        }
    }
}
