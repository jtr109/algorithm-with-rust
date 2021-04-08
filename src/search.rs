use std::slice::Iter;

trait ST<T: PartialEq, U> {
    fn put(&mut self, key: T, value: U);
    fn get(&self, key: T) -> Option<U>;
    fn delete(&mut self, key: T);
    fn contains(&self, key: T) -> bool;
    fn is_empty(&self, key: T) -> bool;
    fn size(&self) -> usize;
    fn keys(&self) -> Iter<T>;
}
