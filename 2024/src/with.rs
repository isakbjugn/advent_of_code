pub trait With<T> {
    fn with(&self, other: &[T]) -> Vec<T>;
}

impl<T: Clone> With<T> for [T] {
    fn with(&self, other: &[T]) -> Vec<T> {
        self.iter().chain(other.iter()).cloned().collect()
    }
}

impl<T: Clone> With<T> for Vec<T> {
    fn with(&self, other: &[T]) -> Vec<T> {
        self.as_slice().with(other)
    }
}