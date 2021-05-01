use crate::{
    edge_weigthed_graph::{Edge, EdgeWeightedGraph},
    min_pq::MinPQ,
};

struct LazyPrimeMST<'a> {
    marked: Vec<bool>,       // 最小生成树的顶点
    mst: Vec<&'a Edge>,      // 最小生成树的边
    pq: MinPQ<'a, &'a Edge>, // 横切边（包括失效的边）
    graph: EdgeWeightedGraph<'a>,
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
}
