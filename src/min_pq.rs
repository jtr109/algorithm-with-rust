pub struct MinPQ<'a, T>
where
    T: PartialOrd,
{
    elements: Vec<&'a T>,
}

impl<'a, T> MinPQ<'a, T>
where
    T: PartialOrd,
{
    pub fn new() -> Self {
        Self { elements: vec![] }
    }

    fn size(&self) -> usize {
        self.elements.len() - 1
    }

    /// 将索引 k 位置的元素上浮到正确位置
    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.elements[k] < self.elements[k / 2] {
            self.elements.swap(k, k / 2);
            k /= 2;
        }
    }

    /// 将索引 k 位置到元素下沉到正确位置
    fn sink(&mut self, mut k: usize) {
        while 2 * k <= self.size() {
            let mut j = 2 * k;
            // 找出 k 的两个（如果有）叶子节点上较小的那个
            if j < self.size() && self.elements[j] > self.elements[j + 1] {
                j += 1;
            }
            if self.elements[k] < self.elements[j] {
                break;
            }
            self.elements.swap(k, j);
            k = j;
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.elements.len() <= 1;
    }

    /// 插入新元素 v
    pub fn insert(&mut self, v: &'a T) {
        if self.elements.len() == 0 {
            self.elements.push(v);
        }
        self.elements.push(v);
        let k = self.size();
        self.swim(k);
    }

    /// 弹出最小值
    pub fn del_min(&mut self) -> Option<&'a T> {
        match self.size() {
            0 => None,
            1 => self.elements.pop(),
            _ => {
                let min = self.elements[1];
                self.elements[1] = self.elements.pop().unwrap();
                self.sink(1);
                Some(min)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn correct_min() {
        let mut pq = MinPQ::new();
        let mut elements: Vec<usize> = (0..100).collect();
        elements.shuffle(&mut thread_rng());
        for e in elements.iter() {
            pq.insert(e);
        }
        for i in 0..100 {
            assert_eq!(pq.del_min(), Some(&i));
        }
    }
}
