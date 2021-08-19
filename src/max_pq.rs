/*!
 * # 最大堆
 *
 * ## 解释
 *
 * ### 数据结构
 *
 * 最大堆是一个处处满足父节点大于子节点的二叉堆。可以通过一个一维矩阵实现，其中 index n 作为父节点，则 index 2n 和 index 2n+1 为两个字节点。需要注意多是，index 0 被我们忽略，没有实际作用。
 *
 * ### 核心方法
 *
 * 核心方法是：
 *
 * 1. 加入新元素：在末尾加入，并使其上浮，直到满足最大二叉堆的数据结构。
 * 2. 删除（提取）最大元素：删除（提取）根节点，将末尾的元素放到根节点（删除原来的末尾元素节点），此时的数据结构不满足最大堆。将根节点当前的元素下沉，直到满足最大堆结构。
 *
 * 在一组父子节点中对目标节点进行上浮（swim）和下沉（sink）使这组节点在变化后实现父节点大于两个字节点。方法如下：
 *
 * - 上浮：将目标节点的值和其父节点比较，如果大于父节点，将目标和父节点交换位置
 * - 下沉：将目标节点的值和其两个子节点比较，如果存在比目标节点值大的子节点，将其中较大的子节点和目标节点交换位置
 *
 * ## 复杂度分析
 *
 * 对于一个含有 $N$ 个元素的优先队列，插入元素操作需要不超过 $(\lg{N}+1)$ 次比较，删除最大元素的操作需要不超过 $2\lg{N}$ 次比较
 */

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
        while 2 * k <= self.size() {
            let mut j = 2 * k;
            if j < self.size() && self.elements[j] < self.elements[j + 1] {
                j += 1;
            }
            if self.elements[k] > self.elements[j] {
                break;
            }
            self.elements.swap(k, j);
            k = j;
        }
    }

    pub fn insert(&mut self, v: T) {
        if self.elements.len() == 0 {
            // 我们的最大堆从 index 1 开始，填充 index 0 用来占位。
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

    pub fn del_max(&mut self) {
        if self.size() > 1 {
            self.elements[1] = self.elements[self.elements.len() - 1];
            self.sink(1);
        }
        self.elements.pop();
    }

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

    #[test]
    fn test_delete_max() {
        let mut pq = MaxPQ {
            elements: vec![1, 10, 8, 9, 4, 7, 6, 5, 2],
        };
        pq.del_max();
        assert_eq!(pq.elements, vec![1, 9, 8, 6, 4, 7, 2, 5]);
    }
}
