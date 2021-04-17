use std::collections::HashSet;

pub struct Graph {
    adj: Vec<HashSet<usize>>,
}

impl Graph {
    fn vertex_count(&self) -> usize {
        1
    }

    fn edge_count(&self) -> usize {
        1
    }

    fn add_edge(&self, v: usize, w: usize) {}

    fn adj(&self, v: usize) -> HashSet<usize> {
        HashSet::new()
    }
}
