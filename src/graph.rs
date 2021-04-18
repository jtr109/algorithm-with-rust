use std::collections::HashSet;

pub struct Graph {
    adj: Vec<HashSet<usize>>,
    vertex: usize,
    edge: usize,
}

impl Graph {
    pub fn new(v: usize) -> Self {
        Graph {
            adj: vec![HashSet::new(); v],
            vertex: v,
            edge: 0,
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex
    }

    pub fn edge_count(&self) -> usize {
        self.edge
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v].insert(w);
        self.adj[w].insert(v);
        self.edge += 1;
    }

    pub fn degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }

    pub fn max_degree(&self) -> usize {
        self.adj.iter().map(|x| x.len()).max().unwrap()
    }

    pub fn avg_degree(&self) -> usize {
        self.adj.iter().map(|x| x.len()).sum::<usize>() / self.adj.len()
    }

    pub fn number_of_self_loops(&self) -> usize {
        self.adj
            .iter()
            .enumerate()
            .filter(|(v, vertexes)| vertexes.get(v).is_some())
            .count()
    }
}
