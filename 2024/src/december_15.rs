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
    0
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
fn sample_input_part_2() {
    let input = include_str!("../input/sample_15_1.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_15.txt");
    assert_eq!(part_1(input), 1457740)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_15.txt");
    assert_eq!(part_2(input), 0)
}
