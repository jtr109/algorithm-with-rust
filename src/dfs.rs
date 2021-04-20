use crate::graph::Graph;

pub struct DepthFirstSearch {
    count: usize,
    marked: Vec<bool>,
    graph: Graph,
}

impl DepthFirstSearch {
    pub fn new(graph: Graph) -> Self {
        DepthFirstSearch {
            count: 0,
            marked: vec![false; graph.vertex_count()],
            graph,
        }
    }

    pub fn dfs(&mut self, s: usize) {
        self.count += 1;
        self.marked[s] = true;
        for v in self.graph.adj(s) {
            if !self.marked[v] {
                self.dfs(v);
            }
        }
    }

    fn marked(&self, s: usize) -> bool {
        self.marked[s]
    }

    /// Get a reference to the depth first search's count.
    pub fn count(&self) -> &usize {
        &self.count
    }
}
