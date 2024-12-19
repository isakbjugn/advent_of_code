
pub trait Sliceable {
    fn substr(&self, start_idx: usize, length: usize) -> Option<&str>;
    fn contains_at(&self, substr: &str, idx: usize) -> bool;
}

impl Sliceable for &str {
    fn substr(&self, start_idx: usize, length: usize) -> Option<&str> {
        let end_idx = start_idx + length;
        if self.len() < end_idx { return None }
        Some(&self[start_idx..end_idx])
    }

    fn contains_at(&self, substr: &str, idx: usize) -> bool {
        matches!(self.substr(idx, substr.len()), Some(str) if str == substr)
    }
}