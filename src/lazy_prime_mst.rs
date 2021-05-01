use crate::{
    edge_weigthed_graph::{Edge, EdgeWeightedGraph},
    min_pq::MinPQ,
    queue::Queue,
};

pub struct LazyPrimeMST<'a> {
    marked: Vec<bool>,    // 最小生成树的顶点
    mst: Queue<&'a Edge>, // 最小生成树的边
    pq: MinPQ<'a, Edge>,  // 横切边（包括失效的边）
    graph: &'a EdgeWeightedGraph<'a>,
}

impl<'a> LazyPrimeMST<'a> {
    /// 标记顶点 v 并将所有连接 v 和未被标记顶点的边加入 pq
    fn visit(&mut self, v: usize) {
        self.marked[v] = true;
        for e in self.graph.adj(v) {
            if self.marked[e.other(v)] {
                continue;
            }
            self.pq.insert(&e);
        }
    }

    pub fn new(graph: &'a EdgeWeightedGraph) -> Self {
        let mut s = Self {
            marked: vec![false; graph.vertex_count()],
            mst: Queue::new(),
            pq: MinPQ::new(),
            graph,
        };
        s.visit(0);
        while !s.pq.is_empty() {
            let e = s.pq.del_min().unwrap();
            let v = e.either();
            let w = e.other(v);
            if s.marked[v] && s.marked[w] {
                continue;
            }
            s.mst.enqueue(e);
            if !s.marked[v] {
                s.visit(w);
            }
            if !s.marked[w] {
                s.visit(v);
            }
        }
        s
    }
}
