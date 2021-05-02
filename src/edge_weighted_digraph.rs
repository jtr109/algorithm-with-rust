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
}
