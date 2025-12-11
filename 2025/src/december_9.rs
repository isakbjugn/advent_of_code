use crate::position::Position;

pub fn part_1(input: &str) -> u64 {
    let mut tiles: Vec<_> = input.lines().map(to_tile).collect();
    tiles.push(tiles[0]);

    let areas = get_rectangles_areas(&tiles);
    areas.into_iter().max().unwrap_or(0)
}

fn to_tile(line: &str) -> Position {
    let mut parts = line.split(',');
    let x: usize = parts.next().unwrap().parse().unwrap();
    let y: usize = parts.next().unwrap().parse().unwrap();
    Position { x, y }
}

fn get_rectangles_areas(tiles: &[Position]) -> Vec<u64> {
    let mut areas = Vec::new();
    for (idx, first_tile) in tiles.iter().enumerate() {
        for second_tile in tiles.iter().skip(idx + 1) {
            let width = first_tile.x.abs_diff(second_tile.x) + 1;
            let height = first_tile.y.abs_diff(second_tile.y) + 1;
            areas.push((width * height) as u64);
        }
    }
    areas
}

pub fn part_2(input: &str) -> u64 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_9.txt");
    assert_eq!(part_1(input), 50)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_9.txt");
    assert_eq!(part_1(input), 4781377701)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_9.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_9.txt");
    assert_eq!(part_2(input), 0)
}
