use itertools::Itertools;
use crate::direction::Direction;
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> u32 {
    let (warehouse_str, movements_str) = input.split_once("\n\n").expect("Could not split input string");
    let mut warehouse = Grid::from_str(warehouse_str).expect("Could not create warehouse grid");
    let movements: Vec<char> = movements_str.split_whitespace().flat_map(|string| string.chars()).collect();

    'movement: for movement in movements {
        let robot_position = warehouse.find('@').expect("No current robot position");
        let direction = match movement {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => unreachable!()
        };
        let direction_vec: Vec<Position> = [robot_position].iter().chain(warehouse.direction_vec(robot_position, direction).iter()).cloned().collect();

        let mut tiles_to_be_moved = Vec::new();
        'direction: for tile in direction_vec.iter() {
            match warehouse.next_value(tile, direction) {
                Some('O') => { tiles_to_be_moved.push(tile) },
                Some('.') => { tiles_to_be_moved.push(tile); break 'direction; },
                Some('#') => { continue 'movement },
                None => unreachable!("How did it come to this?"),
                _ => unreachable!("What on earth could this be?"),
            }
        }

        for tile in tiles_to_be_moved.into_iter().rev() {
            let value = warehouse.get(tile).expect("No valid current value");
            let next_tile = warehouse.next_position(tile, direction).expect("No valid next tile");
            warehouse.set(next_tile, value);
        }
        warehouse.set(robot_position, '.');
    }

    calculate_gps(&warehouse)
}

fn calculate_gps(warehouse: &Grid) -> u32 {
    warehouse.find_iterator('O').map(|Position { x, y}| 100 * y as u32 + x as u32).sum()
}

pub fn part_2(input: &str) -> u32 {
    let (warehouse_str, movements_str) = input.split_once("\n\n").expect("Could not split input string");
    let larger_warehouse_str = warehouse_str
        .replace('#', "##")
        .replace("O", "[]")
        .replace('.', "..")
        .replace('@', "@.");
    
    let mut warehouse = Grid::from_str(&larger_warehouse_str).expect("Could not create warehouse grid");
    // println!("Starting warehouse:\n{}", warehouse);
    
    let movements: Vec<char> = movements_str.split_whitespace().flat_map(|string| string.chars()).collect();

    for (movement_number, movement) in movements.into_iter().enumerate() {
        // println!("Current move: {}", movement);
        let robot_position = warehouse.find('@').expect("No current robot position");
        let direction = match movement {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => unreachable!()
        };

        if let Path::Open(crates_to_move) = find_crates_to_move(&warehouse, robot_position, direction) {
            if (1..crates_to_move.len()).any(|i| crates_to_move[i..].contains(&crates_to_move[i - 1])) {
                // println!("Var duplikater: {:?}", crates_to_move);
                let sorted: Vec<Position> = crates_to_move.clone().into_iter()
                    .unique()
                    .sorted_by(|position_a, position_b| {
                        match direction {
                            Direction::North => position_a.y.cmp(&position_b.y),
                            Direction::South => position_b.y.cmp(&position_a.y),
                            Direction::East => position_b.x.cmp(&position_a.x),
                            Direction::West => position_a.x.cmp(&position_b.x),
                        }
                    })
                    .collect();
                // println!("Sorted: {:?}", sorted);
                for tile in sorted.into_iter() {
                    let value = warehouse.get(&tile).expect("No valid current value");
                    let next_tile = warehouse.next_position(&tile, direction).expect("No valid next tile");
                    warehouse.set(next_tile, value);
                    warehouse.set(tile, '.')
                }
            } else {
                for tile in crates_to_move.into_iter().rev() {
                    let value = warehouse.get(&tile).expect("No valid current value");
                    let next_tile = warehouse.next_position(&tile, direction).expect("No valid next tile");
                    warehouse.set(next_tile, value);
                    warehouse.set(tile, '.')
                }
            }
        }
        // println!("{}", warehouse);
    }

    calculate_larger_gps(&warehouse)
}

enum Path {
    Open(Vec<Position>),
    Blocked,
}

fn find_crates_to_move(warehouse: &Grid, position: Position, direction: Direction) -> Path {
    let direction_vec: Vec<Position> = [position].iter().chain(warehouse.direction_vec(position, direction).iter()).cloned().collect();
    let mut crates_to_move = Vec::new();
    
    for tile in direction_vec {
        match (warehouse.next_value(&tile, direction), direction) {
            (Some('.'), _) => { crates_to_move.push(tile); break },
            (Some('#'), _) => return Path::Blocked,
            (Some('['), Direction::North) |
            (Some('['), Direction::South) => {
                crates_to_move.push(tile);
                let next_tile = warehouse.next_position(&tile, direction).unwrap();
                let other_crate_edge = warehouse.next_position(&next_tile, Direction::East).unwrap();
                match find_crates_to_move(warehouse, other_crate_edge, direction) {
                    Path::Open(path) => path.into_iter().for_each(|tile| crates_to_move.push(tile)),
                    Path::Blocked => return Path::Blocked,
                }
            }
            (Some(']'), Direction::North) |
            (Some(']'), Direction::South) => {
                crates_to_move.push(tile);
                let next_tile = warehouse.next_position(&tile, direction).unwrap();
                let other_crate_edge = warehouse.next_position(&next_tile, Direction::West).unwrap();
                match find_crates_to_move(warehouse, other_crate_edge, direction) {
                    Path::Open(path) => path.into_iter().for_each(|tile| crates_to_move.push(tile)),
                    Path::Blocked => return Path::Blocked,
                }
            }
            (Some('['), _) | (Some(']'), _) => crates_to_move.push(tile),
            _ => unreachable!("How did it come to this, this time?")
        }
    }
    Path::Open(crates_to_move)
}

fn calculate_larger_gps(warehouse: &Grid) -> u32 {
    warehouse.find_iterator('[')
        .map(|Position { x, y}| {
            100 * y as u32 + x as u32
            // match x < (warehouse.width() - 1) / 2 {
            //     true => 100 * y as u32 + x as u32,
            //     false => 100 * y as u32 + x as u32 - 1,
            // }
        }).sum()
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
