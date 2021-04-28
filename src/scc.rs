pub trait SCC {
    fn strongly_connected(v: usize, w: usize) -> bool;
    fn count() -> usize;
    fn id(v: usize) -> usize;
}
