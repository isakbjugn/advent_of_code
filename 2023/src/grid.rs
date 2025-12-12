use std::collections::VecDeque;
use std::fmt;
use std::hash::{Hash, Hasher};
use crate::direction::Direction;
use crate::position::Position;

#[derive(Eq, Clone)]
pub struct Grid {
    pub data: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
}

impl Grid {
    pub fn from_str(input: &str) -> Result<Self, String> {
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

    pub fn new(height: usize, width: usize, fill: char) -> Self {
        let mut rows = Vec::new();

        for _ in 0..height {
            let row: Vec<char> = vec![fill; width];
            rows.push(row);
        }

        Grid { height: rows.len(), width: rows[0].len(), data: rows }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        self.data.get(y).and_then(|row| row.get(x).copied())
    }

    pub fn set(&mut self, position: Position, value: char) {
        if position.x < self.width && position.y < self.height {
            self.data[position.y][position.x] = value;
        } else {
            println!("Trying to set position {:?}, but grid is {}x{}", position, self.width, self.height);
            panic!("Trying to set value outside grid!")
        }
    }

    pub fn neighbor_iter<'a>(&'a self, position: &'a Position) -> impl Iterator<Item = Position> + 'a {
        self.possible_directions(position)
            .into_iter()
            .map(|possible_direction|
                self.next_position(position, possible_direction)
                    .expect("Should be valid position in possible_direction")
            )
    }

    pub fn next_position(&self, position: &Position, direction: Direction) -> Option<Position> {
        let Position { x, y } = *position;
        match direction {
            Direction::North if y > 0 => Some(Position { x, y: y - 1 }),
            Direction::South if y < self.height - 1 => Some(Position { x, y: y + 1 }),
            Direction::East if x < self.width - 1 => Some(Position { x: x + 1, y }),
            Direction::West if x > 0 => Some(Position { x: x - 1, y }),
            _ => None,
        }
    }

    pub fn neighbor_in_direction_from_position(&self, position: Position, direction: Direction) -> Option<Position> {
        // use the existing neighbor_in_direction implementation
        self.next_position(&position, direction)
    }

    pub fn possible_directions(&self, position: &Position) -> Vec<Direction> {
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

    pub fn iter(&self) -> impl Iterator<Item = (Position, char)> + '_ {
        self.data.iter().enumerate().flat_map(move |(row_index, row)| {
            row.iter().enumerate().map(move |(col_index, &value)| {
                (Position { x: col_index, y: row_index }, value)
            })
        })
    }

    pub fn find_iterator(&self, c: char) -> impl Iterator<Item = Position> + '_ {
        self.iter()
            .filter(move |&(_, value)| value == c)
            .map(|(position, _)| position)
    }

    pub fn get_interior_positions(&self, position_inside: &Position) -> Vec<Position> {
        let interior_char = match self.get(position_inside.x, position_inside.y) {
            Some(c) => c,
            None => return Vec::new(), // Return empty if the position is out of bounds
        };
        let mut interior_positions = Vec::new();
        
        let mut visited = VecDeque::new();
        
        visited.push_back(*position_inside);
        interior_positions.push(*position_inside);
        
        while let Some(current_position) = visited.pop_front() {
            for neighbor in self.neighbor_iter(&current_position) {
                if self.get(neighbor.x, neighbor.y) == Some(interior_char) && !interior_positions.contains(&neighbor) {
                    interior_positions.push(neighbor);
                    visited.push_back(neighbor);
                }
            }
        }
        interior_positions
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