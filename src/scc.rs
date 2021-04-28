use crate::depth_first_order::DepthFirstOrder;
use crate::digraph::Digraph;

pub trait SCC {
    fn strongly_connected(&self, v: usize, w: usize) -> bool;
    fn count(&self) -> usize;
    fn id(&self, v: usize) -> usize;
}

pub struct KosarajuSCC<'a> {
    graph: &'a Digraph,
    marked: Vec<bool>,
    ids: Vec<usize>,
    count: usize,
}

impl<'a> KosarajuSCC<'a> {
    pub fn new(graph: &'a Digraph) -> Self {
        let mut s = Self {
            graph,
            marked: vec![false; graph.vertex()],
            ids: vec![0; graph.vertex()],
            count: 0,
        };
        let reversed_graph = &s.graph.reverse();
        let dfo = DepthFirstOrder::new(reversed_graph);
        for v in dfo.reverse_post() {
            if s.marked[v] {
                continue;
            }
            s.dfs(v);
            s.count += 1;
        }
        s
    }

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

impl<'a> SCC for KosarajuSCC<'a> {
    fn strongly_connected(&self, v: usize, w: usize) -> bool {
        self.id(v) == self.id(w)
    }

    fn id(&self, v: usize) -> usize {
        self.ids[v]
    }

    fn count(&self) -> usize {
        self.count
    }
}
