// TODO: implement deref to vec for sorted assertion

pub struct MaxPQ<T: std::cmp::PartialOrd + Copy> {
    elements: Vec<T>,
}

impl<T: std::cmp::PartialOrd + Copy> MaxPQ<T> {
    pub fn new() -> Self {
        MaxPQ { elements: vec![] }
    }

    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.elements[k / 2] < self.elements[k] {
            self.elements.swap(k / 2, k);
            k /= 2;
        }
    }

    fn sink(&mut self, mut k: usize) {
        while 2 * k <= self.size() && self.elements[k] < self.elements[k * 2] {
            self.elements.swap(k, 2 * k);
            k *= 2;
        }
        while 2 * k <= self.size() {
            let mut j = 2 * k;
            if j < self.size() && self.elements[j] < self.elements[j + 1] {
                j += 1;
            }
            self.elements.swap(k, j);
            k = j;
        }
    }

    pub fn insert(&mut self, v: T) {
        if self.elements.len() == 0 {
            self.elements.push(v);
        }
        self.elements.push(v);
        self.swim(self.size());
    }

    pub fn max(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.elements[1])
        }
    }
    pub fn del_max() {}

    pub fn is_empty(&self) -> bool {
        self.size() <= 1
    }

    pub fn size(&self) -> usize {
        let s = self.elements.len();
        if s != 0 {
            s - 1
        } else {
            s
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_swim_on_even_index() {
        let mut pq = MaxPQ {
            elements: vec![0, 1, 2],
        };
        pq.swim(2);
        assert_eq!(pq.elements, vec![0, 2, 1]);
    }

    #[test]
    fn test_swim_on_odd_index() {
        let mut pq = MaxPQ {
            elements: vec![0, 2, 1, 3],
        };
        pq.swim(3);
        assert_eq!(pq.elements, vec![0, 3, 1, 2]);
    }

    #[test]
    fn test_skin() {
        let mut pq = MaxPQ {
            elements: vec![1, 2, 10, 9, 8, 7, 6, 5, 4],
        };
        pq.sink(1);
        assert_eq!(pq.elements, vec![1, 10, 8, 9, 4, 7, 6, 5, 2]);
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
