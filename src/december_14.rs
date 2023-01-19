use std::collections::HashMap;

pub fn part_1(input: &str) -> u32 {
    let mut slice = init_finite_slice(input);
    let mut sand_units = 0;

    'outer: loop {
        let mut current_tile = (500, 0);
        'inner: loop {
            match slice.get(&(current_tile.0, current_tile.1 + 1)).unwrap() {
                Tile::Air => {
                    current_tile.1 += 1;
                    continue 'inner
                },
                Tile::Void => {
                    break 'outer;
                },
                Tile::Rock | Tile::Sand => {
                    match slice.get(&(current_tile.0 - 1, current_tile.1 + 1)).unwrap() {
                        Tile::Air => {
                            current_tile.0 -= 1;
                            current_tile.1 += 1;
                            continue 'inner
                        },
                        Tile::Void => {
                            break 'outer;
                        },
                        Tile::Rock | Tile::Sand => {
                            match slice.get(&(current_tile.0 + 1, current_tile.1 + 1)).unwrap() {
                                Tile::Air => {
                                    current_tile.0 += 1;
                                    current_tile.1 += 1;
                                    continue 'inner
                                },
                                Tile::Void => {
                                    break 'outer;
                                },
                                Tile::Rock | Tile::Sand => {
                                    slice.insert(current_tile, Tile::Sand);
                                    sand_units += 1;
                                    break 'inner
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    sand_units
}

#[derive(Debug)]
enum Tile {
    Air,
    Rock,
    Sand,
    Void,
}

fn init_finite_slice(input: &str) -> HashMap<(u32, u32), Tile> {
    let slice_edges = get_slice_edges(input);
    let mut slice = HashMap::new();
    for col in slice_edges.0..=slice_edges.1 {
        slice.insert((col, slice_edges.2 + 1), Tile::Void);
        for row in 0..=slice_edges.2 {
            slice.insert((col, row), Tile::Air);
        }
    }
    for row in 0..=slice_edges.2 {
        slice.insert((slice_edges.0 - 1, row), Tile::Void);
        slice.insert((slice_edges.1 + 1, row), Tile::Void);
    }

    let rock_tiles = find_rock_tiles(input);

    for tile in rock_tiles {
        slice.insert(tile, Tile::Rock);
    }

    slice
}

fn find_rock_tiles(input: &str) -> Vec<(u32, u32)> {
    let mut rocks = Vec::new();
    let mut x_prev = 500;
    let mut y_prev = 0;

    for line in input.lines() {
        for (idx, coordinate) in line.split(" -> ").enumerate() {
            let mut tuple = coordinate.split(',');
            let (x, y) = (tuple.next().unwrap().parse::<u32>().unwrap(), tuple.next().unwrap().parse::<u32>().unwrap());
            if idx == 0 {
                rocks.push((x, y));
                println!("Inserted rock at: {:?}", (x, y));
                x_prev = x;
                y_prev = y;
                continue
            }

            if x == x_prev {
                if y > y_prev {
                    for row in y_prev+1..=y {
                        rocks.push((x, row));
                        println!("Inserted rock at: {:?}", (x, row));
                    }
                } else {
                    for row in y..y_prev {
                        println!("Inserted rock at: {:?}", (x, row));
                        rocks.push((x, row));
                    }
                }
            }
            if y == y_prev {
                if x > x_prev {
                    for col in x_prev+1..=x {
                        rocks.push((col, y));
                        println!("Inserted rock at: {:?}", (col, y));
                    }
                } else {
                    for col in x..x_prev {
                        rocks.push((col, y));
                        println!("Inserted rock at: {:?}", (col, y));
                    }
                }
            }

            x_prev = x;
            y_prev = y;
        }
    }
    rocks
}

fn init_infinite_slice(input: &str) -> HashMap<(u32, u32), Tile> {
    let mut slice = HashMap::new();
    let rock_tiles = find_rock_tiles(input);

    for tile in rock_tiles {
        slice.insert(tile, Tile::Rock);
    }

    slice
}

fn get_slice_edges(input: &str) -> (u32, u32, u32) {
    let mut left = 500;
    let mut right = 500;
    let mut bottom = 0;
    for line in input.lines() {
        for coordinate in line.split(" -> ") {
            let mut tuple = coordinate.split(',');
            let (x, y) = (tuple.next().unwrap().parse::<u32>().unwrap(), tuple.next().unwrap().parse::<u32>().unwrap());
            if x < left { left = x }
            if x > right { right = x }
            if y > bottom { bottom = y }
        }
    }
    (left, right, bottom)
}

pub fn part_2(input: &str) -> u32 {
    let (_, _, bottom) = get_slice_edges(input);
    let floor = bottom + 2;
    let mut slice = init_infinite_slice(input);
    let mut sand_units = 0;

    'outer: loop {
        if let Some(Tile::Sand) = slice.get(&(500, 0)) {
            break 'outer
        }
        let mut current_tile = (500, 0);

        'inner: loop {
            let next_tile = (current_tile.0, current_tile.1 + 1);
            for col in 0..=2 {
                let this_tile = (next_tile.0 - 1 + col, next_tile.1);
                if next_tile.1 == floor {
                    slice.insert(this_tile, Tile::Rock);
                }
                slice.entry(this_tile).or_insert(Tile::Air);
            }

            match slice.get(&next_tile).unwrap() {
                Tile::Air => {
                    current_tile.1 += 1;
                    continue 'inner
                },
                Tile::Void => (),
                Tile::Rock | Tile::Sand => {
                    match slice.get(&(next_tile.0 - 1, next_tile.1)).unwrap() {
                        Tile::Air => {
                            current_tile.0 -= 1;
                            current_tile.1 += 1;
                            continue 'inner
                        },
                        Tile::Void => (),
                        Tile::Rock | Tile::Sand => {
                            match slice.get(&(next_tile.0 + 1, next_tile.1)).unwrap() {
                                Tile::Air => {
                                    current_tile.0 += 1;
                                    current_tile.1 += 1;
                                    continue 'inner
                                },
                                Tile::Void => (),
                                Tile::Rock | Tile::Sand => {
                                    slice.insert(current_tile, Tile::Sand);
                                    sand_units += 1;
                                    break 'inner
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    sand_units
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_14.txt");
    assert_eq!(part_1(input), 24)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_14.txt");
    assert_eq!(part_2(input), 93)
}
