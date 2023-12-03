use std::collections::{HashMap, HashSet};
use regex::Regex;

pub fn part_1(input: &str) -> i32 {
    let symbol_set = get_symbols(input);

    get_parts(input)
        .iter()
        .filter(|part| symbol_set.is_neighboor(part))
        .map(|part| part.value)
        .sum()
}

struct Part {
    value: i32,
    x_start: usize,
    x_end: usize,
    y: usize,
}

fn get_parts(input: &str) -> Vec<Part> {
    let mut numbers = Vec::new();

    for (y, line) in input.lines().enumerate() {
        Regex::new(r"\d+").unwrap().find_iter(line).for_each(|number| {
            numbers.push(Part {
                value: number.as_str().parse::<i32>().unwrap(),
                x_start: number.start(),
                x_end: number.end(),
                y
            });
        });
    }
    numbers
}

fn get_symbols(input: &str) -> HashSet<(usize, usize)> {
    let mut symbol_set = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            if !symbol.is_ascii_digit() && symbol != '.' {
                symbol_set.insert((x, y));
            }
        }
    }
    symbol_set
}

pub fn part_2(input: &str) -> i32 {
    let mut gears = get_gears(input);

    for part in get_parts(input) {
        for x in part.x_start..part.x_end {
            if gears.is_neighboor_and_push((x, part.y), part.value) {
                break
            }
        }
    }

    gears.iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().product::<i32>()).sum()
}

trait Symbols {
    fn is_neighboor(&self, part: &Part) -> bool;
}

impl Symbols for HashSet<(usize, usize)> {
    fn is_neighboor(&self, part: &Part) -> bool {
        let y = part.y;
        for x in part.x_start..part.x_end {
            if x > 0 {
                if y > 0 && self.contains(&(x-1, y-1)) { return true; }
                if self.contains(&(x-1, y)) { return true; }
                if self.contains(&(x-1, y+1)) { return true; }
            }
            if y > 0 {
                if self.contains(&(x, y-1)) { return true; }
                if self.contains(&(x+1, y-1)) { return true; }
            }
            if self.contains(&(x+1, y)) { return true; }
            if self.contains(&(x, y+1)) { return true; }
            if self.contains(&(x+1, y+1)) { return true; }
        }
        false
    }
}

trait Gears {
    fn is_neighboor_and_push(&mut self, coordinate: (usize, usize), number: i32) -> bool;
}

impl Gears for HashMap<(usize, usize), Vec<i32>> {
    fn is_neighboor_and_push(&mut self, coordinate: (usize, usize), number: i32) -> bool {
        let (x, y) = coordinate;
        if x > 0 {
            if y > 0 && self.contains_key(&(x-1, y-1)) {
                self.get_mut(&(x-1, y-1)).unwrap().push(number);
                return true;
            }
            if self.contains_key(&(x-1, y)) {
                self.get_mut(&(x-1, y)).unwrap().push(number);
                return true;
            }
            if self.contains_key(&(x-1, y+1)) {
                self.get_mut(&(x-1, y+1)).unwrap().push(number);
                return true;
            }
        }
        if y > 0 {
            if self.contains_key(&(x, y-1)) {
                self.get_mut(&(x, y-1)).unwrap().push(number);
                return true;
            }
            if self.contains_key(&(x+1, y-1)) {
                self.get_mut(&(x+1, y-1)).unwrap().push(number);
                return true;
            }
        }
        if self.contains_key(&(x+1, y)) {
            self.get_mut(&(x+1, y)).unwrap().push(number);
            return true;
        }
        if self.contains_key(&(x, y+1)) {
            self.get_mut(&(x, y+1)).unwrap().push(number);
            return true;
        }
        if self.contains_key(&(x+1, y+1)) {
            self.get_mut(&(x+1, y+1)).unwrap().push(number);
            return true;
        }
        false
    }
}

fn get_gears(input: &str) -> HashMap<(usize, usize), Vec<i32>> {
    let mut gear_set = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            if symbol == '*' {
                gear_set.insert((x, y), Vec::new());
            }
        }
    }
    gear_set
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
    assert_eq!(part_2(input), 467835)
}
