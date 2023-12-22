use crate::grid::Grid;

pub fn part_1(input: &str) -> u32 {
    let mut grid = Grid::new(input).unwrap();
    grid.roll(Direction::North);
    grid.calculate_load()
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Grid {
    fn roll(&mut self, direction: Direction) {
        match direction {
            Direction::North | Direction::South => self.roll_columns(direction),
            Direction::East | Direction::West => self.roll_rows(direction),
        }
    }
    // Method to roll all columns north
    fn roll_columns(&mut self, direction: Direction) {
        let cols = self.data.first().map_or(0, Vec::len);

        for col_index in 0..cols {
            // Extracting the column
            let column: Vec<char> = self.data.iter().map(|row| row[col_index]).collect();

            // Applying the roll_north operation
            let rolled_column = roll_along_axis(&column, direction);

            // Placing the modified column back into the grid
            for (row, &new_char) in self.data.iter_mut().zip(rolled_column.iter()) {
                row[col_index] = new_char;
            }
        }
    }
    fn roll_rows(&mut self, direction: Direction) {
        let rows = self.data.len();

        for row_index in 0..rows {
            // Extracting the row
            let row: Vec<char> = self.data[row_index].clone();

            // Applying the roll_north operation
            let rolled_row =  roll_along_axis(&row, direction);

            // Placing the modified row back into the grid
            self.data[row_index] = rolled_row;
        }
    }
    fn calculate_load(&self) -> u32 {
        let height = self.data.len() as u32;
        self.data.iter().enumerate()
            .map(|(i, row)| {
                row.iter()
                    .filter_map(|ch| if *ch == 'O' { Some(height - i as u32) } else { None })
                    .sum::<u32>()
            })
            .sum()
    }
}

fn roll_along_axis(axis: &[char], direction: Direction) -> Vec<char> {
    let new_axis: Vec<Vec<char>> = axis.split_inclusive(|c| c == &'#')
        .map(|slice| {
            if slice[0] == '#' {
                return slice.to_vec();
            }
            match direction {
                Direction::North => roll_slice(slice.to_vec()),
                Direction::South => roll_slice_reverse(slice.to_vec()),
                Direction::East => roll_slice(slice.to_vec()),
                Direction::West => roll_slice_reverse(slice.to_vec()),
            };
            roll_slice(slice.to_vec())
        })
        .collect();
    new_axis.into_iter().flatten().collect()
}

fn roll_slice(mut slice: Vec<char>) -> Vec<char> {
    slice.sort_by_key(|a| order_descending(*a));
    slice
}

fn roll_slice_reverse(mut slice: Vec<char>) -> Vec<char> {
    slice.sort_by_key(|a| order_ascending(*a));
    slice
}

// Helper function to determine the order of a character
fn order_descending(c: char) -> u32 {
    match c {
        'O' => 1,
        '.' => 2,
        '#' => 3,
        _ => 4, // You can add more cases or a default order for other characters
    }
}

// Helper function to determine the order of a character
fn order_ascending(c: char) -> u32 {
    match c {
        '.' => 1,
        'O' => 2,
        '#' => 3,
        _ => 4, // You can add more cases or a default order for other characters
    }
}

fn calculate_load(column: Vec<char>) -> u32 {
    let length = column.len() as u32;
    column.iter().enumerate()
        .filter_map(|(i, ch)| if *ch == 'O' { Some(length - i as u32) } else { None })
        .sum()
}

fn columns_to_vecs(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();
    let max_length = lines[0].chars().count();
    let mut columns = vec![Vec::new(); max_length];

    for line in lines {
        for (i, ch) in line.chars().enumerate() {
            if i < columns.len() {
                columns[i].push(ch);
            }
        }
    }

    columns
}

pub fn part_2(input: &str) -> i16 {

    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_14.txt");
    assert_eq!(part_1(input), 136)
}

#[test]
fn roll_sample_data_north() {
    let sample = include_str!("../input/sample_14.txt");
    let mut sample_rolled_north = Grid::new(sample).unwrap();
    sample_rolled_north.roll(Direction::North);
    let expected_sample_rolled_north = Grid::new(include_str!("../input/sample_14_rolled_north.txt")).unwrap();
    assert_eq!(sample_rolled_north, expected_sample_rolled_north)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_14.txt");
    assert_eq!(part_2(input), 0)
}