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

fn get_antinodes(antenna: &Position, other_antenna: &Position, max_x: usize, max_y: usize) -> Vec<Position> {
    let mut antinodes = Vec::<Position>::new();
    let diff_x: isize = (antenna.x as isize) - (other_antenna.x as isize);
    let diff_y: isize = (antenna.y as isize) - (other_antenna.y as isize);
    let mut x = antenna.x;
    let mut y = antenna.y;
    
    while x < max_x && y < max_y {
        antinodes.push(Position { x, y });

        match (x.checked_add_signed(-diff_x), y.checked_add_signed(-diff_y)) {
            (None, _) | (_, None) => break,
            (Some(new_x), Some(new_y)) => { x = new_x; y = new_y }
        }
    }
    
    x = antenna.x;
    y = antenna.y;

    while x < max_x && y < max_y {
        antinodes.push(Position { x, y });

        match (x.checked_add_signed(diff_x), y.checked_add_signed(diff_y)) {
            (None, _) | (_, None) => break,
            (Some(new_x), Some(new_y)) => { x = new_x; y = new_y }
        }
    }
    
    antinodes
}

pub fn part_2(input: &str) -> usize {
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
                for antinode in get_antinodes(&antenna, &other_antenna, antennas.width, antennas.height) {
                    antinodes.data[antinode.y][antinode.x] = '#';
                }
            }
        }
    }

    antinodes.find_all('#').iter().len()
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_8.txt");
    assert_eq!(part_1(input), 14)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_8.txt");
    assert_eq!(part_2(input), 34)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_8.txt");
    assert_eq!(part_1(input), 280)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_8.txt");
    assert_eq!(part_2(input), 958)
}
