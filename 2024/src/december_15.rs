use std::cmp::Ordering;
use itertools::Itertools;
use crate::direction::Direction;
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> u32 {
    let (warehouse_str, movements_str) = input.split_once("\n\n").expect("Could not split input string");
    let mut warehouse = Grid::from_str(warehouse_str).expect("Could not create warehouse grid");
    let movements: Vec<char> = movements_str.split_whitespace().flat_map(|string| string.chars()).collect();

    for movement in movements {
        let robot_position = warehouse.find('@').expect("No current robot position");
        let direction = match movement {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => unreachable!()
        };
        
        if let Path::Open(crates_to_move) = find_crates_to_move(&warehouse, robot_position, direction) {
            for tile in crates_to_move.into_iter().rev() {
                let value = warehouse.get(&tile).expect("No valid current value");
                let next_tile = warehouse.next_position(&tile, direction).expect("No valid next tile");
                warehouse.set(next_tile, value);
            }
            warehouse.set(robot_position, '.');
        }
    }

    calculate_gps(&warehouse, 'O')
}

fn calculate_gps(warehouse: &Grid, crate_symbol: char) -> u32 {
    warehouse.find_iterator(crate_symbol)
        .map(|Position { x, y}| 100 * y as u32 + x as u32)
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    let (warehouse_str, movements_str) = input.split_once("\n\n").expect("Could not split input string");
    let larger_warehouse_str = warehouse_str
        .replace('#', "##")
        .replace("O", "[]")
        .replace('.', "..")
        .replace('@', "@.");
    
    let mut warehouse = Grid::from_str(&larger_warehouse_str).expect("Could not create warehouse grid");
    
    let movements: Vec<char> = movements_str.split_whitespace().flat_map(|string| string.chars()).collect();

    for movement in movements.into_iter() {
        let robot_position = warehouse.find('@').expect("No current robot position");
        let direction = match movement {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => unreachable!()
        };

        if let Path::Open(crates_to_move) = find_crates_to_move(&warehouse, robot_position, direction) {
            let sorted: Vec<Position> = crates_to_move.clone().into_iter()
                .unique()
                .sorted_by(|a, b| sort_by_direction(a, b, direction))
                .collect();
            for tile in sorted.into_iter() {
                let value = warehouse.get(&tile).expect("No valid current value");
                let next_tile = warehouse.next_position(&tile, direction).expect("No valid next tile");
                warehouse.set(next_tile, value);
                warehouse.set(tile, '.')
            }
        }
    }

    calculate_gps(&warehouse, '[')
}

fn sort_by_direction(rhs: &Position, lhs: &Position, direction: Direction) -> Ordering {
    match direction {
        Direction::North => rhs.y.cmp(&lhs.y),
        Direction::South => lhs.y.cmp(&rhs.y),
        Direction::East => lhs.x.cmp(&rhs.x),
        Direction::West => rhs.x.cmp(&lhs.x),
    }
}

enum Path {
    Open(Vec<Position>),
    Blocked,
}

fn find_crates_to_move(warehouse: &Grid, position: Position, direction: Direction) -> Path {
    let direction_vec: Vec<Position> = [position].iter().chain(warehouse.direction_vec(position, direction).iter()).cloned().collect();
    let mut crates_to_move = Vec::new();
    
    for tile in direction_vec {
        match warehouse.next_value(&tile, direction) {
            Some('O') => crates_to_move.push(tile),
            Some('.') => { crates_to_move.push(tile); break },
            Some('#') => return Path::Blocked,
            Some('[') | Some(']') => {
                crates_to_move.push(tile);
                if matches!(direction, Direction::North | Direction::South) {
                    let next_tile = warehouse.next_position(&tile, direction).unwrap();
                    let other_crate_edge = match warehouse.next_value(&tile, direction) {
                        Some('[') => warehouse.next_position(&next_tile, Direction::East).unwrap(),
                        Some(']') => warehouse.next_position(&next_tile, Direction::West).unwrap(),
                        _ => unreachable!("Invalid crate edge direction"),
                    };
                    match find_crates_to_move(warehouse, other_crate_edge, direction) {
                        Path::Open(path) => crates_to_move.extend(path),
                        Path::Blocked => return Path::Blocked,
                    }
                }
            },
            _ => unreachable!("How did it come to this, this time?")
        }
    }
    Path::Open(crates_to_move)
}

#[test]
fn sample_input_part_1_1() {
    let input = include_str!("../input/sample_15_1.txt");
    assert_eq!(part_1(input), 2028)
}

#[test]
fn sample_input_part_1_2() {
    let input = include_str!("../input/sample_15_2.txt");
    assert_eq!(part_1(input), 10092)
}

#[test]
fn sample_input_part_2_1() {
    let input = include_str!("../input/sample_15_3.txt");
    assert_eq!(part_2(input), 618)
}

#[test]
fn sample_input_part_2_2() {
    let input = include_str!("../input/sample_15_2.txt");
    assert_eq!(part_2(input), 9021)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_15.txt");
    assert_eq!(part_1(input), 1457740)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_15.txt");
    assert_eq!(part_2(input), 1467145)
}
