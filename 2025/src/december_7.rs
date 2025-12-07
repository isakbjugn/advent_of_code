use std::collections::HashSet;
use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> u64 {
    let tachyon_manifold = Grid::from_str(input).unwrap();
    let start_position = tachyon_manifold.find('S').unwrap();

    let mut beams = HashSet::from([start_position.x]);
    let mut splits = 0;

    for y in start_position.y..tachyon_manifold.height() {
        let mut resulting_beams = HashSet::new();
        for &beam_x in &beams {
            match tachyon_manifold.get(&Position { x: beam_x, y: y + 1 }) {
                Some('.') => {
                    resulting_beams.insert(beam_x);
                }
                Some('^') => {
                    resulting_beams.insert(beam_x - 1);
                    resulting_beams.insert(beam_x + 1);
                    splits += 1;
                },
                _ => { /* Strålen går utenfor rutenettet */ }
            };
        }
        beams = resulting_beams;
    }

    splits
}

pub fn part_2(input: &str) -> u64 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_7.txt");
    assert_eq!(part_1(input), 21)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_7.txt");
    assert_eq!(part_1(input), 1675)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_7.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_7.txt");
    assert_eq!(part_2(input), 0)
}
