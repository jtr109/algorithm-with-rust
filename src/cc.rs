use crate::graph::Graph;

pub trait CCAPI<'a> {
    fn new(graph: &'a Graph) -> Self; // 构造函数
    fn connected(&self, v: usize, w: usize) -> bool; //  v 和 w 连通吗
    fn count(&self) -> usize; // 连通分量数
    fn id(&self, v: usize) -> usize; // v 所在的连通分量标识符
}

pub struct CC<'a> {
    marked: Vec<bool>,
    ids: Vec<usize>,
    count: usize,
    graph: &'a Graph,
}

impl<'a> CC<'a> {
    fn dfs(&mut self, v: usize) {
        self.marked[v] = true;
        self.ids[v] = self.count;
        for w in self.graph.adj(v) {
            if self.marked[w] {
                continue;
            }
            self.dfs(w);
        }
    }
}

impl<'a> CCAPI<'a> for CC<'a> {
    fn new(graph: &'a Graph) -> Self {
        let mut cc = Self {
            marked: vec![false; graph.vertex_count()],
            ids: vec![0; graph.vertex_count()],
            count: 0,
            graph,
        };
        for v in 0..graph.vertex_count() {
            if cc.marked[v] {
                continue;
            }
            cc.count += 1;
            cc.dfs(v);
        }
        cc
    }

    fn id(&self, v: usize) -> usize {
        self.ids[v]
    }

    fn connected(&self, v: usize, w: usize) -> bool {
        self.id(v) == self.id(w)
    }

    fn count(&self) -> usize {
        self.count
    }
}
