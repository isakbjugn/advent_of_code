use crate::position::Position;
use crate::grid::Grid;

pub fn part_1(input: &str) -> u64 {
    let (shape_str, region_str) = split_at_first_x_line(input);
    let shapes = to_shapes(shape_str);
    let regions = to_regions(region_str);

    regions.iter().filter(|&region| {
        let mut grid = Grid::new(region.dimensions.y, region.dimensions.x, '.');
        let mut region = region.clone();
        can_fit(&shapes, &mut region, &mut grid)
    }).count() as u64
}

fn split_at_first_x_line(input: &str) -> (&str, &str) {
    // Find the line index of the first line containing 'x'
    let mut byte_index = input.len();

    for (i, line) in input.lines().enumerate() {
        if line.contains('x') {
            // Find byte offset to start of this line
            let mut offset = 0usize;
            for l in input.lines().take(i) {
                offset += l.len() + 1; // +1 for '\n'
            }
            byte_index = offset;
            break;
        }
    }

    input.split_at(byte_index)
}

#[derive(Debug, Clone)]
struct Region {
    dimensions: Position,
    presents: Vec<u8>,
}

fn to_shapes(input: &str) -> Vec<Grid> {
    let mut shapes = Vec::new();
    for shape_str_with_index in input.trim().split("\n\n") {
        let (_, shape_str) = shape_str_with_index.split_once(":").expect("Should be line shift");
        let grid = Grid::from_str(shape_str.trim()).expect("Failed to parse grid");
        shapes.push(grid);
    }
    shapes
}

fn to_regions(input: &str) -> Vec<Region> {
    input
        .lines()
        .map(|line| {
            let (dim_str, presents_str) = line.split_once(':').expect("Invalid region line");
            let dimensions = {
                let mut parts = dim_str.split('x');
                let width: usize = parts.next().unwrap().parse().unwrap();
                let height: usize = parts.next().unwrap().parse().unwrap();
                Position { x: width, y: height }
            };
            let present_counts: Vec<u8> = presents_str.split_whitespace().map(|str| str.parse().expect("Could not parse")).collect();
            let presents = present_counts.iter().enumerate().map(|(index, count)| vec![index as u8; *count as usize]).flatten().collect();
            Region {
                dimensions,
                presents,
            }
        })
        .collect()
}

fn can_fit(shapes: &[Grid], region: &mut Region, grid: &mut Grid) -> bool {
    println!("Trying to fit region with presents {:?} into grid:\n{:?}", region.presents, grid);
    let current_present = match region.presents.pop() {
        None => return true,
        Some(p) => p,
    };
    let current_shape = &shapes[current_present as usize];

    for rotated_shape in current_shape.get_rotations() {
        println!("Trying to fit shape:\n{:?}", rotated_shape);
        for y in 0..=region.dimensions.y - rotated_shape.height() {
            for x in 0..=region.dimensions.x - rotated_shape.width() {
                let position = Position { x, y };
                if can_place(grid, &rotated_shape, &position) {
                    let mut new_grid = grid.place(&rotated_shape, &position);
                    if can_fit(shapes, region, &mut new_grid) {
                        return true;
                    }
                }
            }
        }
    }
    region.presents.push(current_present);
    false
}

fn can_place(grid: &Grid, shape: &Grid, position: &Position) -> bool {
    for y in 0..shape.height() {
        for x in 0..shape.width() {
            let grid_pos = Position { x: position.x + x, y: position.y + y };
            if let Some(grid_value) = grid.get(&grid_pos) {
                let shape_value = shape.get(&Position { x, y }).unwrap();
                if shape_value != '.' && grid_value != '.' {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    true
}

fn place(grid: &mut Grid, shape: &Grid, position: &Position, fill: char) {
    for y in 0..shape.height() {
        for x in 0..shape.width() {
            let grid_pos = Position { x: position.x + x, y: position.y + y };
            let shape_value = shape.get(&Position { x, y }).unwrap();
            if shape_value != '.' {
                grid.set(grid_pos, fill);
            }
        }
    }
}

pub fn part_2(input: &str) -> u64 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_12.txt");
    assert_eq!(part_1(input), 2)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_12.txt");
    assert_eq!(part_1(input), 0)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_12.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_12.txt");
    assert_eq!(part_2(input), 0)
}
