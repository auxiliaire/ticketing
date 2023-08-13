pub trait Getter<F, V> {
    fn get(&self, field: F) -> V;
}
