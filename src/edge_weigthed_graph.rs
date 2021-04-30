pub trait EdgeAPI: std::cmp::PartialOrd {
    fn weight(&self) -> usize; // 边的权重
    fn either(&self) -> usize; // 边两端的顶点之一
    fn other(&self) -> usize; // 另一个顶点
}
