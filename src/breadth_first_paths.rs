use crate::graph::Graph;
use std::collections::VecDeque;

pub struct BreadthFirstPaths {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    start: usize,
    graph: Graph,
}

impl BreadthFirstPaths {
    pub fn new(graph: Graph, start: usize) -> Self {
        let mut breadth_first_search = Self {
            marked: vec![false; graph.vertex_count()],
            edge_to: vec![None; graph.vertex_count()],
            start,
            graph,
        };
        breadth_first_search.bfs();
        breadth_first_search
    }

    fn bfs(&mut self) {
        let mut queue = VecDeque::new();
        queue.push_back(self.start);
        self.marked[self.start] = true;
        while let Some(v) = queue.pop_front() {
            for w in self.graph.adj(v) {
                if self.marked[w] {
                    continue;
                }
                self.edge_to[w] = Some(v);
                self.marked[w] = true;
                queue.push_back(w);
            }
        }
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    pub fn path_to(&self, v: usize) -> Vec<usize> {
        if !self.has_path_to(v) {
            return vec![];
        }
        let mut path = vec![];
        let w = v;
        path.push(w);
        while let Some(w) = self.edge_to[w] {
            path.push(w);
        }
        path.reverse();
        path
    }
}
