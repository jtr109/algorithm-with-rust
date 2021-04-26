use std::collections::HashSet;

pub struct Digraph {
    adj: Vec<HashSet<usize>>,
    vertex: usize,
    edge: usize,
}

impl Digraph {
    pub fn new(v: usize) -> Self {
        Self {
            adj: vec![HashSet::new(); v],
            vertex: v,
            edge: 0,
        }
    }

    pub fn vertex(&self) -> usize {
        self.vertex
    }

    pub fn edge(&self) -> usize {
        self.edge
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v].insert(w);
        self.edge += 1;
    }

    pub fn adj(&self, v: usize) -> HashSet<usize> {
        self.adj[v].clone()
    }

    pub fn reverse(&self) -> Self {
        let mut reversed = Self::new(self.vertex());
        for (v, w_set) in self.adj.iter().enumerate() {
            for w in w_set.iter() {
                reversed.add_edge(*w, v);
            }
        }
        reversed
    }
}
