use std::cmp;
use std::collections::HashSet;
use itertools::Itertools;

pub fn part_1(input: &str) -> u128 {
    calculate_distance(input, 2)
}

fn calculate_distance(input: &str, expansion: u128) -> u128 {
    let grid = to_grid(input);
    let vertical_spaces = vertical_spaces(&grid);
    let horisontal_spaces = horisontal_spaces(&grid);
    let mut distance = 0;

    let galaxies: HashSet<(u16, u16)> = grid.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, &c)| {
            if c == '#' {
                Some((x as u16, y as u16))
            } else {
                None
            }
        })
    }).collect();

    for &galaxy_1 in &galaxies {
        for &galaxy_2 in &galaxies {
            if galaxy_1 != galaxy_2 {
                distance += manhattan_distance(galaxy_1, galaxy_2) as u128
                    + vertical_spaces.iter().filter(|&&x| between(galaxy_1.0, galaxy_2.0, x)).count() as u128 * (expansion - 1)
                    + horisontal_spaces.iter().filter(|&&y| between(galaxy_1.1, galaxy_2.1, y)).count() as u128 * (expansion - 1);
            }
        }
    }

    distance / 2
}

fn between(a: u16, b: u16, c: u16) -> bool {
    (a < c && c < b) || (b < c && c < a)
}

fn horisontal_spaces(grid: &Vec<Vec<char>>) -> Vec<u16> {
    grid.iter().enumerate().filter_map(|(y, row)| {
        if row.iter().all_equal() {
            Some(y as u16)
        } else {
            None
        }
    }).collect()
}

fn vertical_spaces(grid: &Vec<Vec<char>>) -> Vec<u16> {
    let cols = grid[0].len();
    let mut col_to_insert = vec![];

    for col in 0..cols {
        if grid.iter().all(|row| row[col] == '.') {
            col_to_insert.push(col as u16);
        }
    }
    col_to_insert
}

fn manhattan_distance(p1: (u16, u16), p2: (u16, u16)) -> u16 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    // Calculate the Manhattan distance
    (cmp::max(x1, x2) - cmp::min(x1, x2)) + (cmp::max(y1, y2) - cmp::min(y1, y2))
}

fn to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_2(input: &str, expansion: u128) -> u128 {
    calculate_distance(input, expansion)
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_11.txt");
    assert_eq!(part_1(input), 374)
}

#[test]
fn sample_input_part_2_1() {
    let input = include_str!("../input/sample_11.txt");
    assert_eq!(part_2(input, 10), 1030)
}

#[test]
fn sample_input_part_2_2() {
    let input = include_str!("../input/sample_11.txt");
    assert_eq!(part_2(input, 100), 8410)
}