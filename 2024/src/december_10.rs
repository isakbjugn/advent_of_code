use std::collections::HashSet;
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> usize {
    let map = Grid::from_str(input).expect("Could not parse input to grid");

    map.find_iterator('0')
        .map(|trailhead| find_peaks(&trailhead, &map) )
        .flat_map(|peaks| peaks.into_iter())
        .count()
}

fn find_peaks(position: &Position, map: &Grid) -> HashSet<Position> {
    match map.get(position) {
        None => unreachable!("No valid current value"),
        Some('9') => HashSet::from([*position]),
        Some(value) => {
            let height = value.to_digit(10).expect("Could not parse to height");
            map.neighbor_iter(position)
                .filter(|next_position| map.get(next_position).expect("No valid next value").to_digit(10).expect("Could not parse next value to height") == height + 1)
                .map(|next_position| find_peaks(&next_position, map))
                .flat_map(|set| set.into_iter())
                .collect()
        },
    }
}

pub fn part_2(input: &str) -> usize {
    let map = Grid::from_str(input).expect("Could not parse input to grid");

    map.find_iterator('0')
        .map(|trailhead| count_peaks(&trailhead, &map) )
        .sum()
}

fn count_peaks(position: &Position, map: &Grid) -> usize {
    match map.get(position) {
        None => unreachable!("No valid current value"),
        Some('9') => 1,
        Some(value) => {
            let height = value.to_digit(10).expect("Could not parse to height");
            map.neighbor_iter(position)
                .filter(|next_position| map.get(next_position).expect("No valid next value").to_digit(10).expect("Could not parse next value to height") == height + 1)
                .map(|next_position| count_peaks(&next_position, map))
                .sum()
        },
    }
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_10.txt");
    assert_eq!(part_1(input), 36)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_10.txt");
    assert_eq!(part_2(input), 81)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_10.txt");
    assert_eq!(part_1(input), 482)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_10.txt");
    assert_eq!(part_2(input), 1094)
}
