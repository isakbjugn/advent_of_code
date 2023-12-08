use std::collections::HashMap;
use regex::Regex;

pub fn part_1(input: &str) -> i32 {
    let (instructions, node_str) = input.split_once("\n\n").unwrap();
    let instruction_length = instructions.chars().count();
    let nodes: HashMap<&str, (&str, &str)> = node_str.lines()
        .map(|line| {
            if let Some(numbers) = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap().captures(line) {
                let node = numbers.get(1).unwrap().as_str();
                let left = numbers.get(2).unwrap().as_str();
                let right = numbers.get(3).unwrap().as_str();
                (node, (left, right))
            } else { panic!("Invalid string: {}", line) }
        })
        .collect();
    let mut current_node = "AAA";
    let mut steps = 0;

    while current_node != "ZZZ" {
        match instructions.chars().nth(steps % instruction_length) {
            Some('L') => {
                current_node = nodes.get(current_node).unwrap().0;
                steps += 1;
            }
            Some('R') => {
                current_node = nodes.get(current_node).unwrap().1;
                steps += 1;
            }
            _ => panic!("Invalid instruction")
        }
    }

    steps as i32
}

pub fn part_2(input: &str) -> i16 {

    0
}

#[test]
fn sample_input_part_1_1() {
    let input = include_str!("../input/sample_8_1.txt");
    assert_eq!(part_1(input), 2)
}

#[test]
fn sample_input_part_1_2() {
    let input = include_str!("../input/sample_8_2.txt");
    assert_eq!(part_1(input), 6)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_8_2.txt");
    assert_eq!(part_2(input), 0)
}