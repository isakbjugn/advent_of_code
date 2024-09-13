use std::collections::HashMap;
use num::integer::lcm;
use regex::Regex;

pub fn part_1(input: &str) -> i32 {
    let (instructions, node_str) = input.split_once("\n\n").unwrap();
    let instruction_length = instructions.chars().count();
    let nodes = to_nodes(node_str);
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

#[allow(unused)]
trait Nodes {
    fn advance_to_end(&self, start_node: &str, instructions: &str) -> usize;
}

impl Nodes for HashMap<&str, (&str, &str)> {
    fn advance_to_end(&self, start_node: &str, instructions: &str) -> usize {
        let instruction_length = instructions.chars().count();
        let mut current_node = start_node;
        let mut steps = 0;

        while current_node != "ZZZ" {
            match instructions.chars().nth(steps % instruction_length) {
                Some('L') => {
                    current_node = self.get(current_node).unwrap().0;
                    steps += 1;
                }
                Some('R') => {
                    current_node = self.get(current_node).unwrap().1;
                    steps += 1;
                }
                _ => panic!("Invalid instruction")
            }
        }
        steps
    }
}

fn to_nodes(input: &str) -> HashMap<&str, (&str, &str)> {
    input.lines()
        .map(|line| {
            if let Some(numbers) = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap().captures(line) {
                let node = numbers.get(1).unwrap().as_str();
                let left = numbers.get(2).unwrap().as_str();
                let right = numbers.get(3).unwrap().as_str();
                (node, (left, right))
            } else { panic!("Invalid string: {}", line) }
        })
        .collect()
}

pub fn part_2(input: &str) -> i64 {
    let (instructions, node_str) = input.split_once("\n\n").unwrap();
    let instruction_length = instructions.chars().count();
    let nodes = to_nodes(node_str);

    let steps_to_end_node: Vec<i64> = get_start_nodes(&nodes).iter()
        .map(|node| {
            let mut current_node = *node;
            let mut steps = 0;

            while !current_node.ends_with('Z') {
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
            steps as i64
        })
        .collect();
    println!("{:?}", steps_to_end_node);

    get_lowest_common_multiple(&steps_to_end_node)
}

fn get_start_nodes<'a>(nodes: &'a HashMap<&str, (&str, &str)>) -> Vec<&'a str> {
    nodes.iter()
        .filter(|(node, _)| node.ends_with('A'))
        .map(|(node, _)| *node)
        .collect()
}

fn get_lowest_common_multiple(integers: &[i64]) -> i64 {
    let mut multiple: i64 = 1;
    for i in integers {
        multiple = lcm(multiple, *i);
    }
    multiple
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
    let input = include_str!("../input/sample_8_3.txt");
    assert_eq!(part_2(input), 6)
}