pub struct DirectedEdge {
    from: usize,
    to: usize,
    weight: usize,
}

impl DirectedEdge {
    pub fn new(from: usize, to: usize, weight: usize) -> Self {
        Self { from, to, weight }
    }

    pub fn from(&self) -> usize {
        self.from
    }

    pub fn to(&self) -> usize {
        self.to
    }

    pub fn weight(&self) -> usize {
        self.weight
    }
}
