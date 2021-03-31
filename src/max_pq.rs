// TODO: implement deref to vec for sorted assertion

pub struct MaxPQ<T: std::cmp::PartialOrd> {
    pq: Vec<T>,
    n: usize,
}

impl<T: std::cmp::PartialOrd> MaxPQ<T> {
    pub fn new() -> Self {
        MaxPQ { n: 0, pq: vec![] }
    }
    pub fn swim(&mut self, mut k: usize) {
        while k > 1 && self.pq[k / 2] < self.pq[k] {
            self.pq.swap(k / 2, k);
            k /= 2;
        }
    }

    pub fn insert(v: T) {}
    // pub fn max() -> T {}
    pub fn del_max() {}
    pub fn is_empty() -> bool {
        true
    }
    pub fn size() -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_swim_on_even_index() {
        let mut pq = MaxPQ {
            n: 0, // unused
            pq: vec![0, 1, 2],
        };
        pq.swim(2);
        assert_eq!(pq.pq, vec![0, 2, 1]);
    }

    #[test]
    fn test_swim_on_odd_index() {
        let mut pq = MaxPQ {
            n: 0, // unused
            pq: vec![0, 2, 1, 3],
        };
        pq.swim(3);
        assert_eq!(pq.pq, vec![0, 3, 1, 2]);
    }
}
