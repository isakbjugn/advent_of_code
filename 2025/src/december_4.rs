use crate::grid::Grid;

pub fn part_1(input: &str) -> u64 {
    let grid = Grid::from_str(input).unwrap();
    let mut accessible_rolls = 0;

    for (cell, roll) in grid.iter() {
        if roll != '@' { continue };

        if grid.neighbor_iter(&cell).filter(|neighbor| grid.get(neighbor) == Some('@')).count() < 4 {
            accessible_rolls += 1;
            continue;
        }
    }

    accessible_rolls
}

pub fn part_2(input: &str) -> u64 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_4.txt");
    assert_eq!(part_1(input), 13)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_4.txt");
    assert_eq!(part_1(input), 1460)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_4.txt");
    //assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_4.txt");
    //assert_eq!(part_2(input), 0)
}
