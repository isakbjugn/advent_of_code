use std::collections::HashSet;
use itertools::Itertools;
use crate::direction::Direction;
use crate::direction::Direction::{North, South, East, West};
use crate::grid::Grid;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize
}

pub fn part_1(input: &str) -> usize {
    let grid = Grid::new(input).unwrap();
    let mut visited: HashSet<(Position, Direction)> = HashSet::new();
    let mut beams = vec![(Position { x: 0, y: 0 }, East)];
    let mut beam;

    while let Some(new_beam) = beams.pop() {
        beam = new_beam;
        loop {
            if visited.contains(&beam) {
                break;
            } else {
                visited.insert(beam);
                match grid.next_directions(beam.0, beam.1) {
                    (Some(direction_1), Some(direction_2)) => {
                        match (grid.next_position(&(beam.0, direction_1)), grid.next_position(&(beam.0, direction_2))) {
                            (Some(next_position_1), Some(next_position_2)) => {
                                beam = (next_position_1, direction_1);
                                beams.push((next_position_2, direction_2));
                            }
                            (Some(next_position_1), None) => {
                                beam = (next_position_1, direction_1);
                            }
                            (None, Some(next_position_2)) => {
                                beam = (next_position_2, direction_2);
                            }
                            (None, None) => {
                                break;
                            }
                        }
                    }
                    (Some(direction), None) => {
                        match grid.next_position(&(beam.0, direction)) {
                            Some(position) => beam = (position, direction),
                            None => break
                        }
                    }
                    (None, Some(_)) => unreachable!(),
                    (None, None) => break
                }
            }
        }
    }
    visited.iter().map(|(position, _)| position).unique().count()
}

impl Grid {
    fn next_position(&self, beam: &(Position, Direction)) -> Option<Position> {
        let (position, direction) = beam;
        let mut next_position = *position;
        match direction {
            North => {
                if position.y == 0 {
                    return None;
                } else {
                    next_position.y -= 1;
                }
            }
            South => {
                if position.y == self.height - 1 {
                    return None;
                } else {
                    next_position.y += 1;
                }
            }
            East => {
                if position.x == self.width - 1 {
                    return None;
                } else {
                    next_position.x += 1;
                }
            }
            West => {
                if position.x == 0 {
                    return None;
                } else {
                    next_position.x -= 1;
                }
            }
        }
        Some(next_position)
    }
    fn next_directions(&self, position: Position, incoming_direction: Direction) -> (Option<Direction>, Option<Direction>) {
        match self.get(position.x, position.y) {
            Some('.') => (Some(incoming_direction), None),
            Some('-') if [North, South].contains(&incoming_direction) => incoming_direction.perpendicular_option(),
            Some('-') => (Some(incoming_direction), None),
            Some('|') if [East, West].contains(&incoming_direction) => incoming_direction.perpendicular_option(),
            Some('|') => (Some(incoming_direction), None),
            Some('/') => {
                match incoming_direction {
                    North => (Some(East), None),
                    South => (Some(West), None),
                    East => (Some(North), None),
                    West => (Some(South), None),
                }
            }
            Some('\\') => {
                match incoming_direction {
                    North => (Some(West), None),
                    South => (Some(East), None),
                    East => (Some(South), None),
                    West => (Some(North), None),
                }
            }
            _ => (None, None)
        }
    }
}

pub fn part_2(input: &str) -> u32 {

    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_16.txt");
    assert_eq!(part_1(input), 46)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_16.txt");
    assert_eq!(part_2(input), 0)
}