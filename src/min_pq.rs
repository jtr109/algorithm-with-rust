trait MinPQ {
    fn swim(&self, k: usize); // 将索引 k 位置的元素上浮到正确位置
    fn sink(&self, k: usize); // 将索引 k 位置到元素下沉到正确位置
    fn insert(&self, v: usize); // 插入新元素 v
    fn min(&self) -> usize; // 查看优先队列中的最小值
    fn del_min(&self); // 删除优先队列中的最小值
    fn is_empty(&self) -> bool; // 优先队列为空
    fn size(&self) -> usize; // 优先队列中的元素数量
}
