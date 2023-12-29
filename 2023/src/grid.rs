use std::fmt;
use std::hash::{Hash, Hasher};

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