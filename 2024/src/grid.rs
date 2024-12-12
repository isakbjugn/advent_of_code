use std::fmt;
use std::hash::{Hash, Hasher};
use itertools::Itertools;
use crate::direction::Direction;
use crate::position::Position;

#[derive(Eq, Clone)]
pub struct Grid {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
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
    
    pub fn get(&self, position: &Position) -> Option<char> {
        if position.x < self.width && position.y < self.height {
            Some(self.data[position.y][position.x])
        } else {
            None
        }
    }
    
    pub fn height(&self) -> usize { self.height }

    pub fn width(&self) -> usize { self.width }
    
    pub fn set(&mut self, position: Position, value: char) {
        if position.x < self.width && position.y < self.height {
            self.data[position.y][position.x] = value;
        } else {
            panic!("Trying to set value outside grid!")
        }
    }
    
    pub fn next_value(&self, position: &Position, direction: Direction) -> Option<char> {
        self.next_position(position, direction)
            .map(|next_cell| self.get(&next_cell))?
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
    
    pub fn neighbor_iter<'a>(&'a self, position: &'a Position) -> impl Iterator<Item = Position> + 'a {
        self.possible_directions(position)
            .into_iter()
            .map(|possible_direction|
                self.next_position(position, possible_direction)
                    .expect("Should be valid position in possible_direction")
            )
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
    
    pub fn find(&self, c: char) -> Option<Position> {
        self.find_iterator(c).next()
    }

    pub fn find_iterator(&self, c: char) -> impl Iterator<Item = Position> + '_ {
        self.iter()
            .filter(move |&(_, value)| value == c)
            .map(|(position, _)| position)
    }
    
    pub fn direction_vec(&self, position: Position, direction: Direction) -> Vec<Position> {
        let mut positions = Vec::new();
        let mut current_position = position;
        while let Some(next_position) = self.next_position(&current_position, direction) {
            positions.push(next_position);
            current_position = next_position;
        }
        positions
    }

    pub fn iter(&self) -> impl Iterator<Item = (Position, char)> + '_ {
        self.data.iter().enumerate().flat_map(move |(row_index, row)| {
            row.iter().enumerate().map(move |(col_index, &value)| {
                (Position { x: col_index, y: row_index }, value)
            })
        })
    }
    
    pub fn are_neighbors(&self, a: &Position, b: &Position) -> bool {
        self.neighbor_iter(a).contains(b)
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