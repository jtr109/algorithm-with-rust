use std::cmp::Ordering;

#[derive(Eq)]
pub struct Edge {
    weight: usize,
    either: usize,
    other: usize,
}

impl Edge {
    fn new(v: usize, w: usize, weight: usize) -> Self {
        Self {
            weight,
            either: v,
            other: w,
        }
    }

    // 边的权重
    fn weight(&self) -> usize {
        self.weight
    }

    // 边两端的顶点之一
    fn either(&self) -> usize {
        self.either
    }

    // 另一个顶点
    fn other(&self) -> usize {
        self.other
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight().cmp(&other.weight())
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight() == other.weight()
    }
}

trait EdgeWeightedGraphAPI {
    fn vertex_count(&self) -> usize;
    fn edge_count(&self) -> usize;
    fn add_edge(&self, e: Edge) -> usize;
    fn adj(&self, v: usize) -> Vec<Edge>;
    fn edges(&self) -> Vec<Edge>;
}
