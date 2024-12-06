#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    pub fn perpendicular(&self) -> (Direction, Direction) {
        match self {
            Direction::North | Direction::South => (Direction::East, Direction::West),
            Direction::East | Direction::West => (Direction::South, Direction::North),
        }
    }
    pub fn perpendicular_option(&self) -> (Option<Direction>, Option<Direction>) {
        match self {
            Direction::North | Direction::South => (Some(Direction::East), Some(Direction::West)),
            Direction::East | Direction::West => (Some(Direction::South), Some(Direction::North)),
        }
    }
    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}