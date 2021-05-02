use crate::{directed_edge::DirectedEdge, edge_weighted_digraph::EdgeWeightedDigraph};

pub trait SP {
    fn new(graph: &EdgeWeightedDigraph, s: usize) -> Self;
    fn dist_to(&self, v: usize) -> Option<usize>; // 从顶点 s 到 v 的距离
    fn has_path_to(&self, v: usize) -> bool; // 是否存在顶点 s 到 v 到路径
    fn path_to(&self, v: usize) -> Option<Vec<DirectedEdge>>; // 从顶点 s 到 v 的路径
}
