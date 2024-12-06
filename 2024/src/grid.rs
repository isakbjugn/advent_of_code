use std::fmt;
use std::hash::{Hash, Hasher};
use crate::direction;
use crate::direction::Direction;
use crate::position::Position;

#[derive(Eq, Clone)]
pub struct Grid {
    pub data: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
}

impl Grid {
    pub fn new(input: &str) -> Result<Self, String> {
        let mut rows = Vec::new();

        for line in input.lines() {
            // Check if all lines have the same length
            if rows.last().map_or(false, |last_row: &Vec<char>| last_row.len() != line.len()) {
                return Err("Lines of the string must have equal length".to_string());
            }

            let row: Vec<char> = line.chars().collect();
            rows.push(row);
        }

        // Check if the input was not empty
        if rows.is_empty() {
            return Err("Input string is empty".to_string());
        }

        Ok(Grid { height: rows.len(), width: rows[0].len(), data: rows })
    }
    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        self.data.get(y).and_then(|row| row.get(x).copied())
    }
    pub fn get_value_in_direction(&self, x: usize, y: usize, direction: Direction) -> Option<char> {
        self.neighbor_in_direction(x, y, direction)
            .map(|next_cell| self.data[next_cell.1][next_cell.0])
    }
    pub fn neighbor_in_direction(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        match direction {
            Direction::North if y > 0 => Some((x, y - 1)),
            Direction::South if y < self.height - 1 => Some((x, y + 1)),
            Direction::East if x < self.width - 1 => Some((x + 1, y)),
            Direction::West if x > 0 => Some((x - 1, y)),
            _ => None,
        }
    }
    pub fn neighbor_in_direction_from_position(&self, position: Position, direction: Direction) -> Option<Position> {
        // use the existing neighbor_in_direction implementation
        self.neighbor_in_direction(position.x, position.y, direction).map(|(x, y)| Position { x, y })
    }
    pub fn possible_directions(&self, position: Position) -> Vec<Direction> {
        let mut directions = Vec::new();
        if position.y > 0 {
            directions.push(Direction::North);
        }
        if position.y < self.height - 1 {
            directions.push(Direction::South);
        }
        if position.x < self.width - 1 {
            directions.push(Direction::East);
        }
        if position.x > 0 {
            directions.push(Direction::West);
        }
        directions
    }
    
    pub fn find(&self, c: char) -> Option<Position> {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.data[row][col] == c { return Some(Position { x: col, y: row }) }
            }
        }
        None
    }
}

impl Hash for Grid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for row in &self.data {
            for &item in row {
                item.hash(state);
            }
        }
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}