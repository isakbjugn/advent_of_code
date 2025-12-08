#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

impl Position {
    pub fn _manhattan_distance(&self, rhs: Position) -> usize {
        let dx = self.x.abs_diff(rhs.x);
        let dy = self.y.abs_diff(rhs.y);
        dx + dy
    }
}
