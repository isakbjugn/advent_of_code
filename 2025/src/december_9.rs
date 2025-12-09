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
