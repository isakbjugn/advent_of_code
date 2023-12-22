use std::collections::HashMap;
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
    fn cycle(&mut self) {
        self.roll(Direction::North);
        self.roll(Direction::West);
        self.roll(Direction::South);
        self.roll(Direction::East);
    }
    fn roll_columns(&mut self, direction: Direction) {
        let cols = self.data.first().map_or(0, Vec::len);

        for col_index in 0..cols {
            // Finn riktig kolonne
            let column: Vec<char> = self.data.iter().map(|row| row[col_index]).collect();

            // Rull i riktig retning
            let rolled_column = roll_along_axis(&column, direction);

            // Sett inn den nye kolonnen
            for (row, &new_char) in self.data.iter_mut().zip(rolled_column.iter()) {
                row[col_index] = new_char;
            }
        }
    }
    fn roll_rows(&mut self, direction: Direction) {
        let rows = self.data.len();

        for row_index in 0..rows {
            // Finn riktig rad
            let row: Vec<char> = self.data[row_index].clone();

            // Rull i riktig retning
            let rolled_row =  roll_along_axis(&row, direction);

            // Sett inn den nye raden
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
                Direction::West => roll_slice(slice.to_vec()),
                Direction::South => roll_slice_reverse(slice.to_vec()),
                Direction::East => roll_slice_reverse(slice.to_vec()),
            }
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

// Sorter steiner først langs en akse
fn order_descending(c: char) -> u32 {
    match c {
        'O' => 1,
        '.' => 2,
        '#' => 3,
        _ => 4,
    }
}

// Sorter steiner sist langs en akse
fn order_ascending(c: char) -> u32 {
    match c {
        '.' => 1,
        'O' => 2,
        '#' => 3,
        _ => 4,
    }
}

pub fn part_2(input: &str, cycles: u32) -> u32 {
    let mut grid = Grid::new(input).unwrap();
    let mut configurations: HashMap<Grid, u32> = HashMap::new();
    configurations.insert(grid.clone(), 0);

    let mut max_cycles = cycles;
    let mut cycle = 0;
    while cycle < max_cycles {
        grid.cycle();
        if let Some(repeated_state) = configurations.get(&grid) {
            println!("Cycle {} with load {} is a repeat of cycle {}", cycle, grid.calculate_load(), repeated_state);
            max_cycles = lowest_remainder_greater_than_a(cycle, max_cycles, cycle - *repeated_state);
        } else {
            configurations.insert(grid.clone(), cycle);
        }
        cycle += 1;
    }
    grid.calculate_load()
}

fn lowest_remainder_greater_than_a(a: u32, b: u32, c: u32) -> u32 {
    let mut remainder = b % c;
    while remainder <= a {
        remainder += c;
    }
    remainder
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
fn sample_input_part_2_rolled_north_west() {
    let sample = include_str!("../input/sample_14.txt");
    let mut sample_cycle_1 = Grid::new(sample).unwrap();
    sample_cycle_1.roll(Direction::North);
    sample_cycle_1.roll(Direction::West);
    let expected_sample_cycle_1 = Grid::new(include_str!("../input/sample_14_rolled_north_west.txt")).unwrap();

    assert_eq!(sample_cycle_1, expected_sample_cycle_1)
}

#[test]
fn sample_input_part_2_rolled_north_west_south() {
    let sample = include_str!("../input/sample_14.txt");
    let mut sample_cycle_1 = Grid::new(sample).unwrap();
    sample_cycle_1.roll(Direction::North);
    sample_cycle_1.roll(Direction::West);
    sample_cycle_1.roll(Direction::South);
    let expected_sample_cycle_1 = Grid::new(include_str!("../input/sample_14_rolled_north_west_south.txt")).unwrap();

    assert_eq!(sample_cycle_1, expected_sample_cycle_1)
}

#[test]
fn sample_input_part_2_cycle_1() {
    let sample = include_str!("../input/sample_14.txt");
    let mut sample_cycle_1 = Grid::new(sample).unwrap();
    sample_cycle_1.cycle();
    let expected_sample_cycle_1 = Grid::new(include_str!("../input/sample_14_cycle_1.txt")).unwrap();

    assert_eq!(sample_cycle_1, expected_sample_cycle_1)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_14.txt");
    assert_eq!(part_2(input, 1000000000), 64)
}
