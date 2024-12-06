use std::collections::HashSet;
use crate::direction::Direction;
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> usize {
    let grid = Grid::new(input).expect("Unable to create Grid!");
    let mut position = grid.find('^').expect("Could not find starting position");
    let mut visited = HashSet::<Position>::new();
    let mut facing = Direction::North;
    
    loop {
        visited.insert(position);
        match grid.get_value_in_direction(position.x, position.y, facing) {
            Some('.') | Some('^') => position = grid.neighbor_in_direction_from_position(position, facing).expect("Not valid value on grid!"),
            Some('#') => facing = facing.rotate_clockwise(),
            _  => break
        }
    }
    
    visited.len()
}

pub fn part_2(input: &str) -> u32 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_6.txt");
    assert_eq!(part_1(input), 41)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_6.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_6.txt");
    assert_eq!(part_1(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_6.txt");
    assert_eq!(part_2(input), 0)
}
