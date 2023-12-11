use std::collections::HashMap;
use crate::december_10::Direction::{North, South, East, West};
use crate::december_10::Segment::{EndingInInitialDirection, EndingInOppositeDirection, Extending};
use colored::Colorize;
use itertools::Itertools;

pub fn part_1(input: &str) -> u16 {
    let grid = get_grid(input);
    let pipe_loop = get_loop(&grid);
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

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

fn next_pipe_in_direction(pipe: &Position, direction: Direction) -> Position {
    match direction {
        North => Position { x: pipe.x, y: pipe.y - 1 },
        South => Position { x: pipe.x, y: pipe.y + 1 },
        East => Position { x: pipe.x + 1, y: pipe.y },
        West => Position { x: pipe.x - 1, y: pipe.y },
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

fn get_grid(input: &str) -> HashMap<Position, char> {
    let mut grid: HashMap<Position, char> = HashMap::new();

    for (row_idx, line) in input.trim().lines().enumerate() {
        for (col_idx, ch) in line.chars().enumerate() {
            grid.insert(Position { x: col_idx, y: row_idx }, ch);
        }
    }
    grid
}

fn get_loop(grid: &HashMap<Position, char>) -> HashMap<Position, char> {
    let mut pipe_loop: HashMap<Position, char> = HashMap::new();
    let mut current_pipe = grid.iter().find(|&(_, c)| c == &'S').unwrap().0;
    let mut distance = 0;
    let mut previous_direction = South;
    let mut first_direction: Option<Direction> = None;
    let mut last_direction: Option<Direction> = None;

    'grid_loop: while !(grid.get(current_pipe).unwrap() == &'S' && distance != 0) {
        pipe_loop.insert(*current_pipe, *grid.get(current_pipe).unwrap());
        if grid.get(current_pipe).unwrap() == &'S' {
            for direction in [North, South, East, West] {
                if current_pipe.y == 0 && direction == North { continue }
                if let Some(pipe) = grid.get(&next_pipe_in_direction(current_pipe, direction)) {
                    first_direction = Some(direction);
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
            last_direction = Some(direction);
            current_pipe = grid.get_key_value(&next_pipe_in_direction(current_pipe, direction)).unwrap().0;
            previous_direction = direction;
            distance += 1;
        }
    }
    replace_s(
        &mut pipe_loop,
        first_direction.expect("Direction from S was not found"),
        last_direction.expect("Direction to S was not found"));
    //print_grid(grid, &pipe_loop);
    pipe_loop
}

pub fn part_2(input: &str) -> u16 {
    let grid = get_grid(input);
    let (height, width) = grid_dimensions(&grid);
    let pipe_loop = get_loop(&grid);
    let mut num_inside = 0;

    for pipe in grid.keys() {
        let direction = shortest_path_direction(pipe.x, pipe.y, height, width);
        num_inside += is_inside(&pipe_loop, pipe, direction) as u16;
    }
    num_inside
}

fn replace_s(grid: &mut HashMap<Position, char>, first_direction: Direction, last_direction: Direction)  {
    let start = grid.iter().find(|&(_, c)| c == &'S').unwrap().0;
    let start_symbol = match (last_direction, first_direction) {
        (North, South) | (South, North) | (North, North) | (South, South) => '|',
        (East, West) | (West, East) | (East, East) | (West, West) => '-',
        (North, East) | (West, South) => 'F',
        (North, West) | (East, South) => '7',
        (South, East) | (West, North) => 'L',
        (South, West) | (East, North) => 'J',
        _ => unreachable!()
    };
    grid.entry(*start).and_modify(|e| *e = start_symbol);
}

fn shortest_path_out<'a>(
    grid: &'a HashMap<Position, char>,
    pipe: &'a Position,
    direction: Direction,
) -> Vec<(&'a Position, &'a char)> {
    match direction {
        North => Vec::from_iter(grid.iter()
            .filter(|(k, _)| k.x == pipe.x && k.y < pipe.y)
            .sorted_by(|&(a, _), &(b, _)| b.y.cmp(&a.y) )
            .map(|(k, v)| (k, v))),
        South => Vec::from_iter(grid.iter()
            .filter(|(k, _)| k.x == pipe.x && k.y > pipe.y)
            .sorted_by(|&(a, _), &(b, _)| a.y.cmp(&b.y) )
            .map(|(k, v)| (k, v))),
        East => Vec::from_iter(grid.iter()
            .filter(|(k, _)| k.x > pipe.x && k.y == pipe.y)
            .sorted_by(|&(a, _), &(b, _)| a.x.cmp(&b.x) )
            .map(|(k, v)| (k, v))),
        West => Vec::from_iter(grid.iter()
            .filter(|(k, _)| k.x < pipe.x && k.y == pipe.y)
            .sorted_by(|&(a, _), &(b, _)| b.x.cmp(&a.x) )
            .map(|(k, v)| (k, v))),
    }
}

fn is_inside(pipe_loop: &HashMap<Position, char>, pipe: &Position, direction: Direction) -> bool {
    if pipe_loop.get(pipe).is_some() { return false }
    let shortest_path_out = shortest_path_out(pipe_loop, pipe, direction);

    if shortest_path_out.is_empty() { return false }

    let mut inside = false;
    let mut prev_pipe: Option<(&Position, &char)> = None;
    let mut pipe_starting_piece: Option<&char> = None;
    let mut currently_in_pipe = false;

    for current_pipe in shortest_path_out {
        if prev_pipe.is_some() && compatible_neighbors(&current_pipe, &prev_pipe.unwrap(), direction) {
            if !currently_in_pipe {
                pipe_starting_piece = Some(prev_pipe.unwrap().1);
                currently_in_pipe = true;
            }
            let segment_type = segment_type(current_pipe.1, pipe_starting_piece.unwrap(), direction);

            match segment_type {
                EndingInOppositeDirection => inside = !inside,
                Extending | EndingInInitialDirection => (),
            }
        } else {
            currently_in_pipe = false;
            inside = !inside
        }
        prev_pipe = Some(current_pipe);
    }
    inside
}

fn compatible_neighbors(cell_1: &(&Position, &char), cell_2: &(&Position, &char), direction: Direction) -> bool {
    let neighbors = are_neighbors(cell_1.0, cell_2.0);
    let compatible = are_compatible(cell_1.1, cell_2.1, direction);
    neighbors && compatible
}

fn are_neighbors(cell_1: &Position, cell_2: &Position) -> bool {
    cell_1.x == cell_2.x && (cell_1.y as i16 - cell_2.y as i16).abs() == 1 ||
        cell_1.y == cell_2.y && (cell_1.x as i16 - cell_2.x as i16).abs() == 1
}

fn segment_type(pipe: &char, start_pipe: &char, direction: Direction) -> Segment {
    if !are_compatible(pipe, start_pipe, direction) { panic!("Incompatible pipes: {:?} and {:?}", pipe, start_pipe); }
    match direction {
        North | South if pipe == &'|' => Extending,
        North => {
            match (start_pipe, pipe) {
                ('J', 'F') => EndingInInitialDirection,
                ('J', '7') => EndingInOppositeDirection,
                ('L', '7') => EndingInInitialDirection,
                ('L', 'F') => EndingInOppositeDirection,
                _ => unreachable!()
            }
        }
        South => {
            match (start_pipe, pipe) {
                ('F', 'J') => EndingInInitialDirection,
                ('F', 'L') => EndingInOppositeDirection,
                ('7', 'L') => EndingInInitialDirection,
                ('7', 'J') => EndingInOppositeDirection,
                _ => unreachable!()
            }
        }
        East | West if pipe == &'-' => Extending,
        East => {
            match (start_pipe, pipe) {
                ('L', '7') => EndingInInitialDirection,
                ('L', 'J') => EndingInOppositeDirection,
                ('F', 'J') => EndingInInitialDirection,
                ('F', '7') => EndingInOppositeDirection,
                _ => unreachable!()
            }
        }
        West => {
            match (start_pipe, pipe) {
                ('J', 'F') => EndingInInitialDirection,
                ('J', 'L') => EndingInOppositeDirection,
                ('7', 'L') => EndingInInitialDirection,
                ('7', 'F') => EndingInOppositeDirection,
                _ => unreachable!()
            }
        }
    }
}

#[derive(Debug)]
enum Segment {
    EndingInInitialDirection,
    EndingInOppositeDirection,
    Extending,
}

fn are_compatible(pipe: &char, prev_pipe: &char, direction: Direction) -> bool {
    match direction {
        North => ['|', '7', 'F'].contains(pipe) && ['|', 'J', 'L'].contains(prev_pipe),
        South => ['|', 'J', 'L'].contains(pipe) && ['|', '7', 'F'].contains(prev_pipe),
        East => ['-', 'J', '7'].contains(pipe) && ['-', 'F', 'L'].contains(prev_pipe),
        West => ['-', 'F', 'L'].contains(pipe) && ['-', 'J', '7'].contains(prev_pipe)
    }
}

fn shortest_path_direction(x: usize, y: usize, height: usize, width: usize) -> Direction {
    let north = y;
    let south = height - y;
    let west = x;
    let east = width - x;
    match north.min(south).min(west).min(east) {
        min_distance if min_distance == north => North,
        min_distance if min_distance == south => South,
        min_distance if min_distance == east => East,
        min_distance if min_distance == west => West,
        _ => unreachable!(), // This should never happen
    }
}

fn print_grid(grid: &HashMap<Position, char>, pipe_loop: &HashMap<Position, char>) {
    let (height, width) = grid_dimensions(grid);
    for y in 0..height {
        for x in 0..width {
            if let Some(pipe_char) = pipe_loop.get(&Position { x, y }) {
                print!("{}", pipe_char.to_string().green());
            } else if is_inside(pipe_loop, &Position { x, y }, North) {
                print!("{}", ".".red());
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn grid_dimensions(grid: &HashMap<Position, char>) -> (usize, usize) {
    let mut width = 0;
    let mut height = 0;
    for cell in grid.keys() {
        if cell.x > width { width = cell.x }
        if cell.y > height { height = cell.y }
    }
    (height + 1, width + 1)
}

#[test]
fn point_inside_is_inside_north() {
    let mut grid: HashMap<Position, char> = HashMap::new();
    grid.insert(Position { x: 2, y: 0 }, 'F');
    grid.insert(Position { x: 2, y: 1 }, 'J');
    let inside = is_inside(&grid, &Position { x: 2, y: 2 }, North);
    assert!(inside)
}

#[test]
fn point_inside_is_inside_east() {
    let mut grid: HashMap<Position, char> = HashMap::new();
    grid.insert(Position { x: 3, y: 2 }, 'F');
    grid.insert(Position { x: 4, y: 2 }, 'J');
    let inside = is_inside(&grid, &Position { x: 2, y: 2 }, East);
    assert!(inside)
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
    let input = include_str!("../input/sample_10_1.txt");
    assert_eq!(part_2(input), 1)
}

#[test]
fn sample_input_part_2_2() {
    let input = include_str!("../input/sample_10_2.txt");
    assert_eq!(part_2(input), 1)
}

#[test]
fn sample_input_part_2_3() {
    let input = include_str!("../input/sample_10_3.txt");
    assert_eq!(part_2(input), 4)
}

#[test]
fn sample_input_part_2_4() {
    let input = include_str!("../input/sample_10_4.txt");
    assert_eq!(part_2(input), 8)
}

#[test]
fn sample_input_part_2_5() {
    let input = include_str!("../input/sample_10_5.txt");
    assert_eq!(part_2(input), 10)
}