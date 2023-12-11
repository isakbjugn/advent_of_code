use std::collections::HashMap;
use crate::december_10::Direction::{North, South, East, West};

pub fn part_1(input: &str) -> u16 {
    let grid = draw_grid(input);
    let pipe_loop = get_loop(&grid);
    println!("{:?}", pipe_loop);
    pipe_loop.len() as u16 / 2
}

fn possible_pipes_after_start() -> Vec<(Direction, char)> {
    vec![
        (North, '|'), (North, '7'), (North, 'F'),
        (South, '|'), (South, 'L'), (South, 'J'),
        (East, '-'), (East, 'J'), (East, '7'),
        (West, '-'), (West, 'L'), (West, '7'),
    ]
}

fn next_pipe_in_direction(pipe: &(u8, u8), direction: Direction) -> (u8, u8) {
    match direction {
        North => (pipe.0 - 1, pipe.1),
        South => (pipe.0 + 1, pipe.1),
        East => (pipe.0, pipe.1 + 1),
        West => (pipe.0, pipe.1 - 1),
    }
}

fn next_direction(pipe: &char, previous_direction: Direction) -> Option<Direction> {
    match pipe {
        '|' => vec![North, South],
        '-' => vec![East, West],
        'L' => vec![North, East],
        'J' => vec![North, West],
        '7' => vec![South, West],
        'F' => vec![South, East],
        'S' => panic!("Should not find start!"),
        '.' => panic!("Should not find ground!"),
        _ => panic!("Invalid pipe: {}", pipe),
    }.into_iter().find(|&d| d != previous_direction.opposite())
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

fn draw_grid(input: &str) -> HashMap<(u8, u8), char> {
    let mut grid: HashMap<(u8, u8), char> = HashMap::new();

    for (row_idx, line) in input.trim().lines().enumerate() {
        for (col_idx, ch) in line.chars().enumerate() {
            grid.insert((row_idx as u8, col_idx as u8), ch);
        }
    }
    grid
}

fn get_loop(grid: &HashMap<(u8, u8), char>) -> HashMap<(u8, u8), char> {
    let mut pipe_loop: HashMap<(u8, u8), char> = HashMap::new();
    let mut current_pipe = grid.iter().find(|&(_, c)| c == &'S').unwrap().0;
    let mut distance = 0;
    let mut previous_direction = South;

    'grid_loop: while !(grid.get(current_pipe).unwrap() == &'S' && distance != 0) {
        pipe_loop.insert(*current_pipe, *grid.get(current_pipe).unwrap());
        if grid.get(current_pipe).unwrap() == &'S' {
            for direction in [North, South, East, West] {
                if let Some(pipe) = grid.get(&next_pipe_in_direction(current_pipe, direction)) {
                    if possible_pipes_after_start().contains(&(direction, *pipe)) {
                        current_pipe = grid.get_key_value(&next_pipe_in_direction(current_pipe, direction)).unwrap().0;
                        previous_direction = direction;
                        distance += 1;
                        continue 'grid_loop;
                    }
                }
            }
        } else {
            let direction = next_direction(grid.get(current_pipe).unwrap(), previous_direction).unwrap();
            current_pipe = grid.get_key_value(&next_pipe_in_direction(current_pipe, direction)).unwrap().0;
            previous_direction = direction;
            distance += 1;
        }
    }
    pipe_loop
}

pub fn part_2(input: &str) -> u16 {
    let grid = draw_grid(input);
    let pipe_loop = get_loop(&grid);
    let mut num_inside = 0;

    for pipe in grid.keys() {
        num_inside += is_inside(&pipe_loop, pipe) as u16;
    }
    num_inside
}

fn shortest_path_out<'a>(
    grid: &'a HashMap<(u8, u8), char>,
    pipe: &'a (u8, u8),
) -> impl Iterator<Item = (&'a (u8, u8), &'a char)> + 'a {
    let direction = shortest_path_direction(pipe.0 as usize, pipe.1 as usize);
    let condition = move |(k, _): &(&(u8, u8), &char)| match direction {
        North => k.0 == pipe.0 && k.1 < pipe.1,
        South => k.0 == pipe.0 && k.1 > pipe.1,
        East => k.0 > pipe.0 && k.1 == pipe.1,
        West => k.0 < pipe.0 && k.1 == pipe.1,
    };

    grid.iter().filter(condition)
}

fn is_inside(grid: &HashMap<(u8, u8), char>, pipe: &(u8, u8)) -> bool {
    if grid.get(pipe).is_some() { return false }
    shortest_path_out(grid, pipe).count() % 2 == 1
}

fn shortest_path_direction(x: usize, y: usize) -> Direction {
    let north = y;
    let south = 140 - y;
    let west = x;
    let east = 140 - x;
    match north.min(south).min(west).min(east) {
        min_distance if min_distance == north => North,
        min_distance if min_distance == south => South,
        min_distance if min_distance == east => East,
        min_distance if min_distance == west => West,
        _ => unreachable!(), // This should never happen
    }
}

#[test]
fn sample_input_part_1_1() {
    let input = include_str!("../input/sample_10_1.txt");
    assert_eq!(part_1(input), 4)
}

#[test]
fn sample_input_part_1_2() {
    let input = include_str!("../input/sample_10_2.txt");
    assert_eq!(part_1(input), 8)
}

#[test]
fn sample_input_part_2_1() {
    let input = include_str!("../input/sample_10_3.txt");
    assert_eq!(part_2(input), 4)
}

#[test]
fn sample_input_part_2_2() {
    let input = include_str!("../input/sample_10_4.txt");
    assert_eq!(part_2(input), 10)
}