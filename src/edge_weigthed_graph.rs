use std::{cmp::Ordering, collections::HashSet};

#[derive(Eq, Hash)]
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
    pub fn either(&self) -> usize {
        self.v
    }

    // 另一个顶点
    pub fn other(&self, v: usize) -> usize {
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

pub struct EdgeWeightedGraph<'a> {
    vertex_count: usize,
    edge_count: usize,
    adj: Vec<HashSet<&'a Edge>>,
}

impl<'a> EdgeWeightedGraph<'a> {
    pub fn new(vertex_count: usize) -> Self {
        Self {
            vertex_count,
            edge_count: 0,
            adj: vec![HashSet::new(); vertex_count],
        }
    }

    /// 图的顶点数
    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    /// 图的边数
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// 向图中添加一条边 e
    pub fn add_edge(&mut self, e: &'a Edge) {
        let v = e.either();
        let w = e.other(v);
        self.adj[v].insert(e);
        self.adj[w].insert(e);
    }

    /// 和 v 相关联的所有边
    pub fn adj(&self, v: usize) -> HashSet<&'a Edge> {
        self.adj[v].clone()
    }

    /// 图的所有边
    pub fn edges(&self) -> HashSet<&'a Edge> {
        let mut b = HashSet::new();
        for v in 0..self.vertex_count() {
            for e in self.adj(v) {
                if e.other(v) > v {
                    b.insert(e);
                }
            }
        }
        b
    }
}
