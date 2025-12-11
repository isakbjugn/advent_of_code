#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

impl Position {
    pub fn from_xy_tuple(t: (u64, u64)) -> Self {
        Position { x: t.0 as usize, y: t.1 as usize }
    }
    pub fn _manhattan_distance(&self, rhs: Position) -> usize {
        let dx = self.x.abs_diff(rhs.x);
        let dy = self.y.abs_diff(rhs.y);
        dx + dy
    }
    pub fn positions_between(&self, other: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let x_start = self.x.min(other.x);
        let x_end = self.x.max(other.x);
        let y_start = self.y.min(other.y);
        let y_end = self.y.max(other.y);

        for x in x_start..=x_end {
            for y in y_start..=y_end {
                positions.push(Position { x, y });
            }
        }
        positions
    }
}
