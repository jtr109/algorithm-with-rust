use std::usize;

trait UnionFind {
    fn new(n: usize) -> Self;
    fn union(&mut self, p: usize, q: usize); // joining two points
    fn find(&self, p: usize) -> usize; // finding the component ID of a point
    fn connected(&self, p: usize, q: usize) -> bool; // two points are connected
    fn count(&self) -> usize; // the count of components
}

pub struct UF {
    id: Vec<usize>,
    count: usize,
}

impl UnionFind for UF {
    fn new(n: usize) -> Self {
        UF {
            id: (0..n).collect(),
            count: n,
        }
    }

    fn union(&mut self, p: usize, q: usize) {
        if self.connected(p, q) {
            return;
        }
        self.id[p] = q;
        self.count -= 1;
    }

    fn find(&self, p: usize) -> usize {
        if p >= self.id.len() {
            panic!("index out of range")
        }
        let mut i = p;
        while self.id[i] != i {
            i = self.id[i]
        }
        i
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut uf = UF::new(10);
        uf.union(4, 3);
        uf.union(3, 8);
        uf.union(6, 5);
        uf.union(9, 4);
        uf.union(2, 1);
        uf.union(5, 0);
        uf.union(7, 2);
        uf.union(6, 1);
        assert_eq!(uf.count(), 2);
    }
}
