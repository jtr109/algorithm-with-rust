// TODO: implement deref to vec for sorted assertion

pub struct MaxPQ<T: std::cmp::PartialOrd + Copy> {
    elements: Vec<T>,
    max: usize,
}

impl<T: std::cmp::PartialOrd + Copy> MaxPQ<T> {
    pub fn new() -> Self {
        MaxPQ {
            max: 0,
            elements: vec![],
        }
    }
    pub fn swim(&mut self, mut k: usize) {
        while k > 1 && self.elements[k / 2] < self.elements[k] {
            self.elements.swap(k / 2, k);
            k /= 2;
        }
    }

    pub fn insert(&mut self, v: T) {
        if self.elements.len() == 0 {
            self.elements.push(v);
        }
        self.elements.push(v);
        self.max += 1;
        self.swim(self.max);
    }
    // pub fn max() -> T {}
    pub fn del_max() {}
    pub fn is_empty() -> bool {
        true
    }

    pub fn size(&self) -> usize {
        self.max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_swim_on_even_index() {
        let mut pq = MaxPQ {
            max: 0, // unused
            elements: vec![0, 1, 2],
        };
        pq.swim(2);
        assert_eq!(pq.elements, vec![0, 2, 1]);
    }

    #[test]
    fn test_swim_on_odd_index() {
        let mut pq = MaxPQ {
            max: 0, // unused
            elements: vec![0, 2, 1, 3],
        };
        pq.swim(3);
        assert_eq!(pq.elements, vec![0, 3, 1, 2]);
    }

    #[test]
    fn test_insert_first_element() {
        let mut pq = MaxPQ::new();
        assert_eq!(pq.size(), 0);
        pq.insert(1);
        assert_eq!(pq.size(), 1);
        assert_eq!(pq.elements[0], 1);
        assert_eq!(pq.elements[1], 1);
    }

    #[test]
    fn test_insert_element() {
        let mut pq = MaxPQ::new();
        pq.insert(2);
        pq.insert(3);
        assert_eq!(pq.size(), 2);
        assert_eq!(pq.elements[1], 3);
        assert_eq!(pq.elements[2], 2);
    }
}
