use std::cmp::Ordering;

#[derive(Eq)]
pub struct Edge {
    weight: usize,
    v: usize,
    w: usize,
}

impl Edge {
    fn new(v: usize, w: usize, weight: usize) -> Self {
        Self { weight, v, w }
    }

    // 边的权重
    fn weight(&self) -> usize {
        self.weight
    }

    // 边两端的顶点之一
    fn either(&self) -> usize {
        self.v
    }

    // 另一个顶点
    fn other(&self, v: usize) -> usize {
        if self.v == v {
            self.w
        } else if self.w == v {
            self.v
        } else {
            panic!("Inconsistent edge")
        }
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

// struct EdgeWeightedGraph {
//     fn vertex_count(&self) -> usize;
//     fn edge_count(&self) -> usize;
//     fn add_edge(&self, e: Edge) -> usize;
//     fn adj(&self, v: usize) -> Vec<Edge>;
//     fn edges(&self) -> Vec<Edge>;
// }
