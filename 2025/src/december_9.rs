use crate::position::Position;

pub fn part_1(input: &str) -> u64 {
    let tiles: Vec<_> = input.lines().map(to_tile).collect();
    let areas = get_rectangles_areas(&tiles);
    areas.into_iter().max().unwrap_or(0)
}

fn to_tile(line: &str) -> (u64, u64) {
    let mut parts = line.split(',');
    let x: u64 = parts.next().unwrap().parse().unwrap();
    let y: u64 = parts.next().unwrap().parse().unwrap();
    (x, y)
}

fn get_rectangles_areas(tiles: &[(u64, u64)]) -> Vec<u64> {
    let mut areas = Vec::new();
    for i in 0..tiles.len() {
        for j in i+1..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];
            let width = x1.abs_diff(x2) + 1;
            let height = y1.abs_diff(y2) + 1;
            areas.push(width * height);
        }
    }
    areas
}

pub fn part_2(input: &str) -> u64 {
    let mut tiles: Vec<_> = input.lines().map(to_tile).collect();
    tiles.push(tiles[0]);

    if tiles.len() < 10 {
        let edge_tiles = tiles
            .windows(2)
            .map(|w| (Position::from_xy_tuple(w[0]), Position::from_xy_tuple(w[1])))
            .flat_map(|(first_tile, second_tile)| first_tile.positions_between(&second_tile))
            .map(|pos| (pos.x as u64, pos.y as u64))
            .collect::<Vec<_>>();

        let mut areas = Vec::new();
        for top_left in &tiles {
            for bottom_right in tiles.iter().filter(|tile| tile.0 > top_left.0 && tile.1 > top_left.1) {
                let area = (bottom_right.0 - top_left.0 + 1) * (bottom_right.1 - top_left.1 + 1);

                let mut valid = true;
                for &edge_tile in &edge_tiles {
                    let x_inside = edge_tile.0 > top_left.0 && edge_tile.0 < bottom_right.0;
                    let y_inside = edge_tile.1 > top_left.1 && edge_tile.1 < bottom_right.1;
                    if x_inside && y_inside {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    areas.push(area);
                }
            }
        }
        return areas.into_iter().max().unwrap_or(0);
    }

    let mut top_right = None;
    for two_tiles in tiles.windows(2) {
        let first = two_tiles[0];
        let second = two_tiles[1];
        if second.0 > first.0 && second.0 - first.0 > 20000 {
            top_right = Some(two_tiles[1])
        }
    }

    let mut areas = Vec::new();

    let top_right = top_right.expect("Fant ikke top right corner");
    for bottom_left in tiles.iter().filter(|&&(x, y)| x < top_right.0 && y > top_right.1) {
        let upper_left = (bottom_left.0, top_right.1);
        let lower_right = (top_right.0, bottom_left.1);
        let area = (lower_right.0 - upper_left.0 + 1) * (lower_right.1 - upper_left.1 + 1);

        let mut valid = true;
        for &tile in &tiles {
            let x_inside = tile.0 > upper_left.0 && tile.0 < lower_right.0;
            let y_inside = tile.1 > upper_left.1 && tile.1 < lower_right.1;
            if x_inside && y_inside {
                valid = false;
                break;
            }
        }
        if valid {
            areas.push(area);
        }
    }

    areas.into_iter().max().unwrap_or(0)
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
    assert_eq!(part_2(input), 1470616992)
}
