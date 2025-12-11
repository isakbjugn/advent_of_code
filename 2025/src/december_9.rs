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
    let mut tiles: Vec<_> = input.lines().map(to_tile).collect();
    tiles.push(tiles[0]);

    let edge_tiles = tiles
        .windows(2)
        .map(|w| (w[0], w[1]))
        .flat_map(|(first_tile, second_tile)| first_tile.positions_between(&second_tile))
        .collect::<Vec<_>>();

    let mut areas = Vec::new();
    for top_left in &tiles {
        for bottom_right in tiles.iter().filter(|tile| tile.x > top_left.x && tile.y > top_left.y) {
            let area = (bottom_right.x - top_left.x + 1) as u64 * (bottom_right.y - top_left.y + 1) as u64;

            if !edge_tiles.iter().any(|tile| is_tile_inside_area(tile, top_left, bottom_right)) {
                areas.push(area);
            }
        }
    }

    areas.into_iter().max().unwrap_or(0)
}

fn is_tile_inside_area(tile: &Position, top_left: &Position, bottom_right: &Position) -> bool {
    tile.x > top_left.x && tile.x < bottom_right.x &&
        tile.y > top_left.y && tile.y < bottom_right.y
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
    assert_eq!(part_2(input), 24)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_9.txt");
    assert_eq!(part_2(input), 0)
}
