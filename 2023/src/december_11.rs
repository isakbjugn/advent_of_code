use std::cmp;
use std::collections::HashSet;

pub fn part_1(input: &str) -> u128 {
    let mut grid = to_grid(input);
    let mut distance = 0;
    expand_vertically(&mut grid);
    expand_horisontally(&mut grid);

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
                distance += manhattan_distance(galaxy_1, galaxy_2) as u128;
            }
        }
    }

    distance / 2
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

fn expand_vertically(grid: &mut Vec<Vec<char>>) {
    let mut y = 0;
    while y < grid.len() {
        if grid[y].iter().all(|&c| c == '.') {
            let duplicate_line = grid[y].clone();
            y += 1; // Increment to insert after the current line
            grid.insert(y, duplicate_line);
        }
        y += 1;
    }
}

fn expand_horisontally(grid: &mut Vec<Vec<char>>) {
    let rows = grid.len();
    if rows == 0 {
        return;
    }

    let cols = grid[0].len();
    let mut col_to_insert = vec![];

    for col in 0..cols {
        if grid.iter().all(|row| row[col] == '.') {
            col_to_insert.push(col);
        }
    }

    for &col in col_to_insert.iter().rev() {
        for row in grid.iter_mut() {
            row.insert(col, '.');
        }
    }
}

pub fn part_2(input: &str) -> i16 {

    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_11.txt");
    assert_eq!(part_1(input), 374)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_11.txt");
    assert_eq!(part_2(input), 0)
}