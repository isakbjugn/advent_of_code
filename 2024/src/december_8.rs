use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> usize {
    let antennas = Grid::new(input).expect("Could not parse antennas to grid");
    let mut antinodes = Grid::from_char(antennas.height, antennas.width, '.');

    for row in 0..antennas.height {
        for col in 0..antennas.width {
            let antenna = Position { x: col, y: row };
            let frequency = match antennas.get_value(&antenna) {
                None => unreachable!("Antenna position not in grid"),
                Some('.') => continue,
                Some(c) => c
            };

            let other_antennas = antennas.find_all(frequency);
            // println!("For antenna {:?} found other antennas with same frequency: {:?}", antenna, other_antennas);

            for other_antenna in other_antennas {
                if other_antenna == antenna { continue }
                if let Some(antinode) = get_antinode(&antenna, &other_antenna) {
                    if antennas.get_value(&antinode).is_some() {
                        antinodes.data[antinode.y][antinode.x] = '#';
                    }
                }
            }
        }
    }

    antinodes.find_all('#').iter().len()
}

fn get_antinode(antenna: &Position, other_antenna: &Position) -> Option<Position> {
    let x = (antenna.x + antenna.x).checked_sub(other_antenna.x);
    let y = (antenna.y + antenna.y).checked_sub(other_antenna.y);

    match (x, y) {
        (None, _) | (_, None) => None,
        (Some(x), Some(y)) => Some(Position { x, y })
    }
}

pub fn part_2(input: &str) -> u64 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_8.txt");
    assert_eq!(part_1(input), 14)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_8.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_8.txt");
    assert_eq!(part_1(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_8.txt");
    assert_eq!(part_2(input), 0)
}
