use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> usize {
    let (locks, keys) = to_locks_and_keys(input);
    locks.into_iter().map(|lock| {
        keys.iter()
            .filter(|key| {
                !(0..5).map(|index| lock[index] + key[index]).any(|sum| sum > 5)
            })
            .count()
    }).sum()
}

enum Security {
    LOCK,
    KEY
}

fn to_locks_and_keys(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut locks: Vec<Vec<u8>> = Vec::new();
    let mut keys: Vec<Vec<u8>> = Vec::new();
    input.split("\n\n")
        .map(|element| {
            let lock_or_key = match element.lines().next().expect("There is a first line").starts_with("#####") {
                true => Security::LOCK,
                false => Security::KEY,
            };
            (element, lock_or_key)
        })
        .map(|(element, lock_or_key)| {
            let grid = Grid::from_str(element).expect("Can make grid of lock or key");
            let mut heights: Vec<u8> = vec![0; 5];
            for column in 0..5 {
                for row in 1..6 {
                    if grid.get(&Position { x: column, y: row }) == Some('#') {
                        heights[column] += 1;
                    }
                }
            }
            (heights, lock_or_key)
        })
        .for_each(|(heights, lock_or_key)| {
            match lock_or_key {
                Security::LOCK => locks.push(heights),
                Security::KEY => keys.push(heights),
            }
        });
        (locks, keys)
}

pub fn part_2(input: &str) -> u64 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_25.txt");
    assert_eq!(part_1(input), 3)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_25.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_25.txt");
    assert_eq!(part_1(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_25.txt");
    assert_eq!(part_2(input), 0)
}
