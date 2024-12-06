use std::collections::HashSet;
use itertools::Itertools;
use crate::direction::Direction;
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> usize {
    find_path(input).len()
}

fn find_path(input: &str) -> HashSet<Position> {
    let grid = Grid::new(input).expect("Unable to create Grid!");
    let mut position = grid.find('^').expect("Could not find starting position");
    let mut visited = HashSet::<Position>::new();
    let mut facing = Direction::North;

    loop {
        visited.insert(position);
        match grid.next_value(&position, facing) {
            Some('.') | Some('^') => position = grid.next_position(position, facing).expect("Not valid value on grid!"),
            Some('#') => facing = facing.rotate_clockwise(),
            _  => break
        }
    }
    visited
}

pub fn part_2(input: &str) -> usize {
    let visited = find_path(input);

    let grid = Grid::new(input).expect("Unable to create Grid!");
    let starting_position = grid.find('^').expect("Could not find starting position");
    let starting_direction = Direction::North;
    
    visited.into_iter().filter(|position| {
        let grid = {
            let mut grid = grid.clone();
            grid.data[position.y][position.x] = '#';
            grid
        };
        
        let mut previous_positions_directions = HashSet::<(Position, Direction)>::new();
        
        let mut position = starting_position;
        let mut facing = starting_direction;

        while let Some(next_cell) = grid.next_position(position, facing) {
            if previous_positions_directions.iter().contains(&(position, facing)) {
                return true
            } else {
                previous_positions_directions.insert((position, facing));
            }
            
            match grid.get_value(&next_cell) {
                Some('#') => facing = {
                    facing.rotate_clockwise()
                },
                Some('.') | Some('^') => {
                    position = next_cell;
                }
                _ => { unreachable!() }
            }
        }
        false
    }).count()
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
