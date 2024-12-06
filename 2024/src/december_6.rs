use std::collections::HashSet;
use crate::direction::Direction;
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> usize {
    let grid = Grid::from_str(input).expect("Unable to create Grid!");
    let mut position = grid.find('^').expect("Could not find starting position");
    let mut visited = HashSet::<Position>::new();
    let mut facing = Direction::North;

    loop {
        visited.insert(position);
        match grid.next_value(&position, facing) {
            Some('.') | Some('^') => position = grid.next_position(&position, facing).expect("Not valid value on grid!"),
            Some('#') => facing = facing.rotate_clockwise(),
            _  => break
        }
    }
    visited.len()
}

pub fn part_2(input: &str) -> usize {
    let grid = Grid::from_str(input).expect("Unable to create Grid!");
    let mut position = grid.find('^').expect("Could not find starting position");
    let mut facing = Direction::North;

    let mut faced_obstacles = HashSet::<(Position, Direction)>::new(); // obstacle and incoming direction
    let mut loop_positions = HashSet::<Position>::new();

    while let Some(next_cell) = grid.next_position(&position, facing) {
        match grid.get(&next_cell) {
            Some('#') => facing = {
                faced_obstacles.insert((next_cell, facing));
                facing.rotate_clockwise()
            },
            Some('.') | Some('^') => {
                if leads_to_loop(position, facing, &grid, faced_obstacles.clone()) {
                    loop_positions.insert(next_cell);
                }
                position = next_cell;
            }
            _ => { unreachable!() }
        }
    }

    loop_positions.len()
}

fn leads_to_loop(position: Position, originally_facing: Direction, grid: &Grid, mut faced_obstacles: HashSet<(Position, Direction)>) -> bool {
    let mut position = position;
    let mut facing = originally_facing.rotate_clockwise();

    faced_obstacles.insert((grid.next_position(&position, originally_facing).unwrap(), originally_facing));

    while let Some(next_cell) = grid.next_position(&position, facing) {
        if faced_obstacles.contains(&(next_cell, facing)) {
            return true;
        }
        match grid.get(&next_cell) {
            Some('#') => facing = {
                faced_obstacles.insert((next_cell, facing));
                facing.rotate_clockwise()
            },
            Some('.') | Some('^') => {
                position = next_cell;
            }
            _ => { unreachable!() }
        }
    }
    false
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_6.txt");
    assert_eq!(part_1(input), 41)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_6.txt");
    assert_eq!(part_2(input), 6)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_6.txt");
    assert_eq!(part_1(input), 4778)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_6.txt");
    assert_eq!(part_2(input), 1618)
}
