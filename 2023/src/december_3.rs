use std::collections::HashSet;

pub fn part_1(input: &str) -> i32 {
    let symbol_set = symbol_coordinates(input);
    let valid_coordinates_set = valid_part_coordinates(symbol_set);
    let mut sum: i32 = 0;
    let mut prev_was_number = false;

    for (y, line) in input.lines().enumerate() {
        for (start_index, symbol) in line.chars().enumerate() {
            if symbol.is_ascii_digit() && !prev_was_number {
                prev_was_number = true;
                let end_index = line[start_index..].find(|c: char| !c.is_ascii_digit()).unwrap_or(line[start_index..].len()) + start_index;
                let number = match end_index {
                    _ if end_index == start_index => line[start_index..=start_index].parse::<i32>().unwrap_or_else(|_| panic!("{} {} {}", start_index, end_index, line)),
                    _ => line[start_index..end_index].parse::<i32>().unwrap_or_else(|_| panic!("{} {} {}", start_index, end_index, line)),
                };
                for x in start_index..end_index {
                    if valid_coordinates_set.contains(&(x, y)) {
                        sum += number;
                        break
                    }
                }
            }
            else {
                prev_was_number = symbol.is_ascii_digit();
            }
        }
    }

    sum
}

fn symbol_coordinates(input: &str) -> HashSet<(usize, usize)> {
    let mut symbol_set = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            if !symbol.is_ascii_digit() && symbol != '.' {
            //if symbol != '.' {
                symbol_set.insert((x, y));
            }
        }
    }
    symbol_set
}

fn valid_part_coordinates(input: HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut valid_coordinates_set = HashSet::new();
    for (x, y) in input {
        if x > 0 && y > 0 { valid_coordinates_set.insert((x-1, y-1)); }
        if y > 0 {
            valid_coordinates_set.insert((x, y-1));
            valid_coordinates_set.insert((x+1, y-1));
        }
        if x > 0 {
            valid_coordinates_set.insert((x-1, y));
            valid_coordinates_set.insert((x-1, y+1));
        }
        valid_coordinates_set.insert((x+1, y));
        valid_coordinates_set.insert((x, y+1));
        valid_coordinates_set.insert((x+1, y+1));
    }
    valid_coordinates_set
}

pub fn part_2(_input: &str) -> i32 {

    0
}
#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_3.txt");
    assert_eq!(part_1(input), 4361)
}

#[test]
fn sample_input_part_1_1() {
    let input = include_str!("../input/sample_3_1.txt");
    assert_eq!(part_1(input), 15420)
}

#[test]
fn sample_input_part_1_2() {
    let input = include_str!("../input/sample_3_2.txt");
    assert_eq!(part_1(input), 9559)
}

#[test]
fn sample_input_part_1_3() {
    let input = include_str!("../input/sample_3_3.txt");
    assert_eq!(part_1(input), 16773)
}

#[test]
fn sample_input_part_2_1() {
    let input = include_str!("../input/sample_3.txt");
    assert_eq!(part_2(input), 0)
}