use crate::direction::Direction;
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> u64 {
    let dig_instructions: Vec<(Direction, u64)> = input
        .lines()
        .map(to_dig_instructions)
        .collect();

    let (grid_width, grid_height, min_x, min_y) = get_dimensions(&dig_instructions);

    let mut grid = Grid::new(grid_height + 1, grid_width + 1, '.');
    let mut current_position = Position { x: (0 - min_x) as usize, y: (0 - min_y) as usize };
    grid.set(current_position, '#');

    for (direction, length) in dig_instructions {
        for _ in 0..length {
            current_position = grid.next_position(&current_position, direction).unwrap();
            grid.set(current_position, '#');
        }
    }

    let position_inside = Position { x: (0 - min_x) as usize + 1, y: (0 - min_y) as usize + 1 };

    let interior_positions = grid.get_interior_positions(&position_inside);
    for position in interior_positions {
        grid.set(position, '#');
    }

    grid.find_iterator('#').count() as u64
}

fn to_dig_instructions(str: &str) -> (Direction, u64) {
    let str_split_by_whitespace: Vec<&str> = str.split_whitespace().collect();
    let direction = match str_split_by_whitespace[0] {
        "U" => Direction::North,
        "D" => Direction::South,
        "R" => Direction::East,
        "L" => Direction::West,
        _ => panic!("Unknown direction"),
    };
    let length: u64 = str_split_by_whitespace[1].parse().unwrap();
    (direction, length)
}

fn get_dimensions(dig_instructions: &[(Direction, u64)]) -> (usize, usize, i64, i64) {
    let mut current_x: i64 = 0;
    let mut current_y: i64 = 0;
    let mut min_x: i64 = 0;
    let mut max_x: i64 = 0;
    let mut min_y: i64 = 0;
    let mut max_y: i64 = 0;

    for (direction, length) in dig_instructions {
        match direction {
            Direction::North => current_y -= *length as i64,
            Direction::South => current_y += *length as i64,
            Direction::East => current_x += *length as i64,
            Direction::West => current_x -= *length as i64,
            _ => panic!("Unsupported direction in dig instructions"),
        }

        if current_x < min_x {
            min_x = current_x;
        }
        if current_x > max_x {
            max_x = current_x;
        }
        if current_y < min_y {
            min_y = current_y;
        }
        if current_y > max_y {
            max_y = current_y;
        }
    }

    let width = (max_x - min_x) as usize;
    let height = (max_y - min_y) as usize;

    (width, height, min_x, min_y)
}

pub fn part_2(input: &str) -> u64 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_18.txt");
    assert_eq!(part_1(input), 62)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_18.txt");
    assert_eq!(part_1(input), 40714)
}

#[test]
fn sample_input_part() {
    let input = include_str!("../input/sample_18.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_18.txt");
    assert_eq!(part_2(input), 0)
}
