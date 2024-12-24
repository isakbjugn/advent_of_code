use std::collections::HashMap;
use itertools::Itertools;

#[derive(Eq, Hash, PartialEq)]
#[derive(Debug)]
enum Operator {
    AND,
    OR,
    XOR,
}

pub fn part_1(input: &str) -> u64 {
    let mut wires: HashMap<&str, Option<u8>> = HashMap::new();
    let mut gates: Vec<(&str, Operator, &str, &str)> = Vec::new();
    
    let (wires_input, gates_input) = input.split_once("\n\n").expect("Input has one two-line break");
    wires_input.lines()
        .map(|line| line.split_once(": ").expect("Wire input has \": \""))
        .for_each(|(wire, initial_value)| {
            wires.insert(wire, Some(initial_value.parse::<u8>().expect("Wire value can be parsed as u8")));
        });
    
    // Please help write a Regex to parse this line into ((&str, Operator, &str), &str):
    // x00 AND y00 -> z00
    let re = regex::Regex::new(r"([a-z0-9]+) (AND|OR|XOR) ([a-z0-9]+) -> ([a-z0-9]+)").unwrap();
    
    gates_input.lines()
        .map(|line| {
            if let Some(captures) = re.captures(line) {
                let wire_1 = captures.get(1).expect("First wire").as_str();
                let operator = match captures.get(2).expect("Operator").as_str() {
                    "AND" => Operator::AND,
                    "OR" => Operator::OR,
                    "XOR" => Operator::XOR,
                    _ => panic!("Unknown operator"),
                };
                let wire_2 = captures.get(3).expect("Second wire").as_str();
                let wire_3 = captures.get(4).expect("Third wire").as_str();
                (wire_1, operator, wire_2, wire_3)
            } else {
                panic!("Could not parse line")
            }
        })
        .for_each(|(wire_1, operator, wire_2, wire_3)| {
            if !wires.contains_key(wire_1) {
                wires.insert(wire_1, None);
            }
            if !wires.contains_key(wire_2) {
                wires.insert(wire_2, None);
            }
            if !wires.contains_key(wire_3) {
                wires.insert(wire_3, None);
            }
            gates.push((wire_1, operator, wire_2, wire_3));
        });
    
    while wires.values().contains(&None) {
        for (input_1, operator, input_2, output) in gates.iter() {
            let output_wire_value = match (wires.get(input_1).unwrap(), operator, wires.get(input_2).unwrap()) {
                (Some(input_wire_1), Operator::AND, Some(input_wire_2)) => {
                    input_wire_1 & input_wire_2
                },
                (Some(input_wire_1), Operator::OR, Some(input_wire_2)) => {
                    input_wire_1 | input_wire_2
                },
                (Some(input_wire_1), Operator::XOR, Some(input_wire_2)) => {
                    input_wire_1 ^ input_wire_2
                },
                _ => continue
            };
            let output_wire = wires.get_mut(output).unwrap();
            *output_wire = Some(output_wire_value);
        }
    }
    
    wires.into_iter()
        .filter(|(wire, _)| wire.starts_with('z'))
        .sorted_by_key(|kv| kv.0)
        .rev()
        // .for_each(|kv| println!("{}: {}", kv.0, kv.1.unwrap()));
        .fold(0, |acc, (_, elem)| acc * 2 + elem.expect("Every wire should have a value by now") as u64)
    // 0
}

pub fn part_2(input: &str) -> u32 {
    0
}

#[test]
fn sample_input_part_1_1() {
    let input = include_str!("../input/sample_24_1.txt");
    assert_eq!(part_1(input), 4)
}

#[test]
fn sample_input_part_1_2() {
    let input = include_str!("../input/sample_24_2.txt");
    assert_eq!(part_1(input), 2024)
}

#[test]
fn sample_input_part_2_1() {
    let input = include_str!("../input/sample_24_1.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_24.txt");
    assert_eq!(part_1(input), 51837135476040)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_24.txt");
    assert_eq!(part_2(input), 0)
}
