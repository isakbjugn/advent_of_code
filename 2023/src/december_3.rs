use std::collections::HashSet;
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
    x: usize,
    y: usize,
}

impl Part {
    fn len(&self) -> usize {
        self.value.ilog10() as usize
    }
    fn in_hitbox(&self, coordinate: &(usize, usize)) -> bool {
        let x_start = if self.x > 0 { self.x - 1 } else { self.x };
        let y_start = if self.y > 0 { self.y - 1 } else { self.y };
        (x_start..=self.x +self.len()+1).contains(&coordinate.0)
            && (y_start..=self.y+1).contains(&coordinate.1)
    }
}

fn get_parts(input: &str) -> Vec<Part> {
    let mut numbers = Vec::new();

    for (y, line) in input.lines().enumerate() {
        Regex::new(r"\d+").unwrap().find_iter(line).for_each(|number| {
            numbers.push(Part {
                value: number.as_str().parse::<i32>().unwrap(),
                x: number.start(),
                y
            });
        });
    }
    numbers
}

#[derive(Eq, PartialEq, Hash)]
struct Symbol {
    value: char,
    x: usize,
    y: usize,
}

impl Symbol {
    fn get_coordinate(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

fn get_symbols(input: &str) -> HashSet<Symbol> {
    let mut symbol_set = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            if !symbol.is_ascii_digit() && symbol != '.' {
                symbol_set.insert(Symbol { value: symbol, x, y });
            }
        }
    }
    symbol_set
}

fn get_gears(input: &str) -> HashSet<Symbol> {
    get_symbols(input).into_iter().filter(|symbol| symbol.value == '*').collect()
}

pub fn part_2(input: &str) -> i32 {
    let parts = get_parts(input);

    let gears: Vec<Vec<i32>> = get_gears(input)
        .iter()
        .map(|gear| Vec::from_iter(
            parts
                .iter()
                .filter(|part| part.in_hitbox(&gear.get_coordinate()))
                .map(|part| part.value)
        ))
        .collect();

    gears
        .iter()
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts.iter().product::<i32>())
        .sum()
}

trait Symbols {
    fn is_neighboor(&self, part: &Part) -> bool;
}

impl Symbols for HashSet<Symbol> {
    fn is_neighboor(&self, part: &Part) -> bool {
        self.iter().any(|symbol| part.in_hitbox(&symbol.get_coordinate()))
    }
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
