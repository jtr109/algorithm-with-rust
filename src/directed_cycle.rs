use crate::digraph::Digraph;

pub struct DirectedCycle<'a> {
    graph: &'a Digraph,
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    cycle: Vec<usize>,
    on_stack: Vec<bool>,
}

impl<'a> DirectedCycle<'a> {
    pub fn new(graph: &'a Digraph) -> Self {
        let mut dc = Self {
            graph,
            marked: vec![false; graph.vertex()],
            edge_to: vec![None; graph.vertex()],
            cycle: vec![],
            on_stack: vec![false; graph.vertex()],
        };
        for v in 0..graph.vertex() {
            if dc.marked[v] {
                continue;
            }
            dc.dfs(v);
        }
        dc
    }

    fn has_cycle(&self) -> bool {
        !self.cycle.is_empty()
    }

    fn marked(&self, v: usize) -> bool {
        self.marked[v]
    }

    fn on_stack(&self, v: usize) -> bool {
        self.on_stack[v]
    }

    fn dfs(&mut self, v: usize) {
        self.on_stack[v] = true;
        self.marked[v] = true;
        for w in self.graph.adj(v) {
            if self.has_cycle() {
                return;
            }
            if !self.marked(v) {
                self.edge_to[w] = Some(v);
                self.dfs(w);
            } else if self.on_stack(v) {
                self.cycle = vec![];
                let mut x = v;
                while x != w {
                    self.cycle.push(x);
                    x = self.edge_to[x].unwrap();
                }
                self.cycle.push(x);
            }
        }
        self.on_stack[v] = false;
    }
}
