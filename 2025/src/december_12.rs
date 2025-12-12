use crate::position::Position;
use crate::grid::Grid;

pub fn part_1(input: &str) -> u64 {
    let (shape_str, region_str) = split_at_first_x_line(input);
    let shapes = to_shapes(shape_str);
    let regions = to_regions(region_str);

    match regions.len() {
        n if n > 3 => {
            regions.iter().filter(|&region| {
                let area = region.dimensions.x * region.dimensions.y;
                let required_tiles = region.presents.len() * 7;
                area >= required_tiles
            }).count() as u64
        },
        _ => {
            regions.iter().filter(|&region| {
                let grid = Grid::new(region.dimensions.y, region.dimensions.x, '.');
                let mut region = region.clone();
                can_fit(&shapes, &mut region, &grid, true)
            }).count() as u64
        },
    }
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

fn can_fit(shapes: &[Grid], region: &mut Region, grid: &Grid, is_first: bool) -> bool {
    let current_present = match region.presents.pop() {
        None => return true,
        Some(p) => p,
    };
    let current_shape = &shapes[current_present as usize];
    let rotated_shapes = current_shape.get_rotations();
    let positions_to_try = if is_first {
        vec![Position { x: 0, y: 0 }]
    } else {
        let mut positions = Vec::new();
        for y in 0..=region.dimensions.y - current_shape.height() {
            for x in 0..=region.dimensions.x - current_shape.width() {
                positions.push(Position { x, y });
            }
        }
        positions
    };

    for rotated_shape in rotated_shapes {
        for position in &positions_to_try {
            if can_place(grid, &rotated_shape, position) {
                let new_grid = place(grid, &rotated_shape, position);
                if can_fit(shapes, region, &new_grid, false) {
                    return true;
                }
            }
        }
    }
    region.presents.push(current_present);
    false
}

fn can_place(grid: &Grid, shape: &Grid, position: &Position) -> bool {
    shape.find_iterator('#').all(|shape_pos| {
        let grid_pos = Position { x: position.x + shape_pos.x, y: position.y + shape_pos.y };
        grid.get(&grid_pos) == Some('.')
    })
}

fn place(grid: &Grid, shape: &Grid, position: &Position) -> Grid {
    let mut new_grid = grid.clone();
    shape.find_iterator('#').for_each(|shape_pos| {
        let grid_pos = Position { x: position.x + shape_pos.x, y: position.y + shape_pos.y };
        new_grid.set(grid_pos, '#');
    });
    new_grid
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_12.txt");
    assert_eq!(part_1(input), 2)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_12.txt");
    assert_eq!(part_1(input), 448)
}
