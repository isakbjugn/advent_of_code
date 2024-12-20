use rayon::prelude::*;
use crate::direction::Direction;
use crate::grid::Grid;
use crate::position::Position;
use rustc_hash::FxHashSet;

pub fn part_1(input: &str) -> usize {
    let grid = Grid::from_str(input).expect("Unable to create Grid!");
    let mut position = grid.find('^').expect("Could not find starting position");
    let mut visited = FxHashSet::<Position>::default();
    let mut facing = Direction::North;

    loop {
        visited.insert(position);
        match grid.next_value(&position, facing) {
            Some('.') | Some('^') => position = grid.next_position(&position, facing).expect("Not valid value on grid!"),
            Some('#') => facing = facing.clockwise(),
            _  => break
        }
    }
    visited.len()
}

pub fn part_2(input: &str) -> usize {
    let grid = Grid::from_str(input).expect("Unable to create Grid!");
    let mut position = grid.find('^').expect("Could not find starting position");
    let mut not_visited: Vec<Vec<bool>> = vec![vec![true; grid.width()]; grid.height()];
    let mut facing = Direction::North;
    let mut faced_obstacles: FxHashSet<(Position, Direction)> = FxHashSet::default();
    
    let mut should_check: Vec<(Position, Direction, FxHashSet<(Position, Direction)>)> = Vec::new();

    while let Some(next_position) = grid.next_position(&position, facing) {
        not_visited[position.y][position.x] = false;
        match grid.get(&next_position) {
            Some('.') | Some('^') => {
                let next_position = grid.next_position(&position, facing).expect("Not valid value on grid!");
                if not_visited[next_position.y][next_position.x] {
                    should_check.push((position, facing, faced_obstacles.clone()))
                }
                position = next_position;
            },
            Some('#') => {
                let obstacle = grid.next_position(&position, facing).expect("Not valid value on grid!");
                faced_obstacles.insert((obstacle, facing));
                facing = facing.clockwise()
            },
            _  => unreachable!()
        }
    }
    
    should_check.into_par_iter()
        .filter(|args| {
            let next_position = grid.next_position(&args.0, args.1).expect("Should be valid next position");
            leads_to_loop(args.0, args.1, next_position, &grid, args.2.clone())
        })
        .count()
}

fn leads_to_loop(mut position: Position, mut facing: Direction, obstacle: Position, grid: &Grid, mut faced_obstacles: FxHashSet<(Position, Direction)>) -> bool {
    while let Some(next_cell) = grid.next_position(&position, facing) {
        if faced_obstacles.contains(&(next_cell, facing)) {
            return true
        }
        
        if next_cell == obstacle {
            faced_obstacles.insert((next_cell, facing));
            facing = facing.clockwise();
            continue
        }

        match grid.get(&next_cell) {
            Some('#') => {
                faced_obstacles.insert((next_cell, facing));
                facing = facing.clockwise();
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
