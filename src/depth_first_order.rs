use crate::digraph::Digraph;

pub struct DepthFirstOrder<'a> {
    graph: &'a Digraph,
    marked: Vec<bool>,
    pre: Vec<usize>,
    post: Vec<usize>,
}

impl<'a> DepthFirstOrder<'a> {
    pub fn new(graph: &'a Digraph) -> Self {
        let mut s = Self {
            graph,
            marked: vec![false; graph.vertex()],
            pre: vec![],
            post: vec![],
        };
        for v in 0..graph.vertex() {
            if s.marked[v] {
                continue;
            }
            s.dfs(v);
        }
        s
    }

    fn dfs(&mut self, v: usize) {
        self.marked[v] = true;
        for w in self.graph.adj(v) {
            if self.marked[w] {
                continue;
            }
            self.dfs(w);
        }
    }

    pub fn pre(&self) -> Vec<usize> {
        self.pre.clone()
    }

    pub fn post(&self) -> Vec<usize> {
        self.post.clone()
    }

    pub fn reverse_post(&self) -> Vec<usize> {
        let mut rev = self.post.clone();
        rev.reverse();
        rev
    }
}
