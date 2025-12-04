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
    let mut grid = Grid::from_str(input).unwrap();
    let mut accessible_rolls = 0;
    let mut removed_last_round = true;
    let mut remove_list = Vec::new();

    while removed_last_round {
        removed_last_round = false;
        for (cell, roll) in grid.iter() {
            if roll != '@' { continue };

            if grid.neighbor_iter(&cell).filter(|neighbor| grid.get(neighbor) == Some('@')).count() < 4 {
                accessible_rolls += 1;
                remove_list.push(cell);
                removed_last_round = true;
                continue
            }
        }
        for &cell in &remove_list {
            grid.set(cell, 'x');
        }
    }

    accessible_rolls
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
    assert_eq!(part_2(input), 43)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_4.txt");
    assert_eq!(part_2(input), 9243)
}
