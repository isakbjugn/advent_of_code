use crate::grid::Grid;
use crate::position::Position;

pub fn part_1(input: &str) -> usize {
    let antennas = Grid::from_str(input).expect("Could not parse antennas to grid");
    let mut antinodes = Grid::new(antennas.height(), antennas.width(), '.');
    
    antennas.iter()
        .filter(|&(_, value)| value != '.')
        .for_each(|(antenna, frequency)| {
            antennas.find_iterator(frequency)
                .filter(|&other_antenna| other_antenna != antenna)
                .for_each(|other_antenna| {
                    if let Some(antinode) = get_antinode(&antenna, &other_antenna) {
                        if antennas.get(&antinode).is_some() {
                            antinodes.set(antinode, '#');
                        }
                    }
                });
        });

    antinodes.find_iterator('#').count()
}

fn get_antinode(antenna: &Position, other_antenna: &Position) -> Option<Position> {
    let x = (antenna.x + antenna.x).checked_sub(other_antenna.x);
    let y = (antenna.y + antenna.y).checked_sub(other_antenna.y);

    match (x, y) {
        (None, _) | (_, None) => None,
        (Some(x), Some(y)) => Some(Position { x, y })
    }
}

pub fn part_2(input: &str) -> usize {
    let antennas = Grid::from_str(input).expect("Could not parse antennas to grid");
    let mut antinodes = Grid::new(antennas.height(), antennas.width(), '.');

    antennas.iter()
        .filter(|&(_, value)| value != '.')
        .for_each(|(antenna, frequency)| {
            antennas.find_iterator(frequency)
                .filter(|&other_antenna| other_antenna != antenna)
                .for_each(|other_antenna| {
                    for antinode in get_antinodes(&antenna, &other_antenna, antennas.width(), antennas.height()) {
                        antinodes.set(antinode, '#');
                    }
                });
        });

    antinodes.find_iterator('#').count()
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
