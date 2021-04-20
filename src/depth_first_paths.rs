use crate::graph::Graph;

struct DepthFirstPaths {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    start: usize,
    graph: Graph,
}

impl DepthFirstPaths {
    fn new(graph: Graph, start: usize) -> Self {
        Self {
            marked: vec![false; graph.vertex_count()],
            edge_to: vec![None; graph.vertex_count()],
            start,
            graph,
        }
    }

    fn dfs(&mut self, v: usize) {
        self.marked[v] = true;
        for w in self.graph.adj(v) {
            if self.marked[w] {
                continue;
            }
            self.edge_to[w] = Some(v);
            self.dfs(w);
        }
    }

    fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    fn path_to(&self, v: usize) -> Vec<usize> {
        if !self.has_path_to(v) {
            return vec![];
        }
        let mut w = v;
        let mut path = vec![];
        path.push(w);
        while w != self.start {
            w = self.edge_to[w].unwrap();
            path.push(w)
        }
        path
    }
}
