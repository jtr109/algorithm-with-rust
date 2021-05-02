use std::collections::HashSet;

use crate::directed_edge::DirectedEdge;

pub struct EdgeWeightedDigraph<'a> {
    vertex_count: usize,
    edge_count: usize,
    adj: Vec<HashSet<&'a DirectedEdge>>,
}

impl<'a> EdgeWeightedDigraph<'a> {
    pub fn new(v: usize) -> Self {
        Self {
            vertex_count: v,
            edge_count: 0,
            adj: vec![HashSet::new(); v],
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn add_edge(&mut self, e: &'a DirectedEdge) {
        self.adj[e.from()].insert(&e);
        self.edge_count += 1;
    }

    pub fn adj(&self, v: usize) -> &HashSet<&DirectedEdge> {
        &self.adj[v]
    }

    pub fn edges(&self) -> HashSet<&DirectedEdge> {
        let mut result = HashSet::new();
        for v in 0..self.vertex_count() {
            for e in self.adj(v) {
                result.insert(*e);
            }
        }
        result
    }
}
