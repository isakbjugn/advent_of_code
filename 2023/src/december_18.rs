use crate::direction::Direction;

#[derive(Clone, Copy)]
pub struct PositionI64 {
    pub x: i64,
    pub y: i64
}

pub fn part_1(input: &str) -> i64 {
    let dig_instructions: Vec<_> = input
        .lines()
        .map(to_dig_instruction)
        .collect();

    let start_position = PositionI64 { x: 0, y: 0 };
    let mut current_position = start_position;

    let mut vertices = vec![current_position];
    let mut perimeter = 0i64;

    for (direction, length) in dig_instructions {
        perimeter += length;
        let (x, y) = (current_position.x, current_position.y);
        match direction {
            Direction::North => current_position = PositionI64 { x, y: y - length },
            Direction::South => current_position = PositionI64 { x, y: y + length },
            Direction::East => current_position = PositionI64 { x: x + length, y },
            Direction::West => current_position = PositionI64 { x: x - length, y },
        }
        vertices.push(current_position);
    }

    // Pick's theorem: A = i + b/2 - 1
    // A = area, i = interior cells, b = boundary cells
    // We want i + b (total cells), which equals: A + b/2 + 1
    shoelace_area(&vertices) + perimeter / 2 + 1
}

fn to_dig_instruction(str: &str) -> (Direction, i64) {
    let str_split_by_whitespace: Vec<&str> = str.split_whitespace().collect();
    let direction = match str_split_by_whitespace[0] {
        "U" => Direction::North,
        "D" => Direction::South,
        "R" => Direction::East,
        "L" => Direction::West,
        _ => panic!("Unknown direction"),
    };
    let length = str_split_by_whitespace[1].parse().unwrap();
    (direction, length)
}

fn shoelace_area(vertices: &[PositionI64]) -> i64 {
    vertices
        .windows(2)
        .map(|windows| (windows[0], windows[1]))
        .map(|(start, end)| (start.y + end.y) * (start.x - end.x))
        .sum::<i64>()
        .abs() / 2
}

pub fn part_2(input: &str) -> i64 {
    let dig_instructions: Vec<_> = input
        .lines()
        .map(to_dig_instruction_hex)
        .collect();

    let start_position = PositionI64 { x: 0, y: 0 };
    let mut current_position = start_position;

    let mut vertices = vec![current_position];
    let mut perimeter = 0i64;

    for (direction, length) in dig_instructions {
        perimeter += length;
        let (x, y) = (current_position.x, current_position.y);
        match direction {
            Direction::North => current_position = PositionI64 { x, y: y - length },
            Direction::South => current_position = PositionI64 { x, y: y + length },
            Direction::East => current_position = PositionI64 { x: x + length, y },
            Direction::West => current_position = PositionI64 { x: x - length, y },
        }
        vertices.push(current_position);
    }

    // Pick's theorem: A = i + b/2 - 1
    // A = area, i = interior cells, b = boundary cells
    // We want i + b (total cells), which equals: A + b/2 + 1
    shoelace_area(&vertices) + perimeter / 2 + 1
}

fn to_dig_instruction_hex(str: &str) -> (Direction, i64) {
    let str_split_by_whitespace = str.split_once('#').unwrap().1.trim_matches(')');
    let (distance_hex, direction_hex) = str_split_by_whitespace.split_at(5);
    let distance = i64::from_str_radix(distance_hex, 16).unwrap();
    let direction = match direction_hex {
        "0" => Direction::East,
        "1" => Direction::South,
        "2" => Direction::West,
        "3" => Direction::North,
        _ => panic!("Unknown direction"),
    };
    (direction, distance)
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
fn test_dig_instruction_hex() {
    assert_eq!(to_dig_instruction_hex("(#70c710)"), (Direction::East, 461937))
}

#[test]
fn sample_input_part() {
    let input = include_str!("../input/sample_18.txt");
    assert_eq!(part_2(input), 952408144115)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_18.txt");
    assert_eq!(part_2(input), 129849166997110)
}
