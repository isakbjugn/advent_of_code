use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Operator {
    AND,
    OR,
    XOR,
}

pub fn part_1(input: &str) -> u64 {
    let (wires, gates) = to_wires_and_gates(input);
    perform_calculation(&wires, &gates).expect("Part 1 should have a solution")
}

pub fn part_2(input: &str) -> String {
    let (wires, gates) = to_wires_and_gates(input);

    let x = calculate_decimal(&wires, 'x');
    let y = calculate_decimal(&wires, 'y');

    // Atomic flag for early termination
    let found = AtomicBool::new(false);
    let result = Mutex::new(String::new());

    unique_k_pair_combinations(&Vec::from_iter(0..gates.len()), 4)
        .par_bridge() // Convert to parallel iterator
        .for_each(|permutation| {
            // println!("{:?}", permutation);
            if found.load(Ordering::Relaxed) {
                return; // Exit early if a solution is found
            }
    
            let mut gates = gates.clone();
            swap_wires(&mut gates, permutation[0].0, permutation[1].1);
            swap_wires(&mut gates, permutation[1].0, permutation[1].1);
            swap_wires(&mut gates, permutation[2].0, permutation[2].1);
            swap_wires(&mut gates, permutation[3].0, permutation[3].1);
    
            if let Some(z) = perform_calculation(&wires, &gates) {
                if x + y == z {
                    let solution = [
                        gates[permutation[0].0].3,
                        gates[permutation[0].1].3,
                        gates[permutation[1].0].3,
                        gates[permutation[1].1].3,
                        gates[permutation[2].0].3,
                        gates[permutation[2].1].3,
                        gates[permutation[3].0].3,
                        gates[permutation[4].1].3,
                    ]
                        .into_iter()
                        .sorted()
                        .join(",");
    
                    *result.lock().unwrap() = solution;
                    found.store(true, Ordering::Relaxed);
                }
            }
        });

    let result = result.lock().unwrap().clone();
    result
}

fn swap_wires(gates: &mut Vec<(&str, Operator, &str, &str)>, i: usize, j: usize) {
    let (a, b) = if i < j {
        let (first, rest) = gates.split_at_mut(j);
        (&mut first[i], &mut rest[0])
    } else {
        let (first, rest) = gates.split_at_mut(i);
        (&mut rest[0], &mut first[j])
    };

    let temp = a.3;
    a.3 = b.3;
    b.3 = temp;
}

fn to_wires_and_gates(input: &str) -> (HashMap<&str, Option<u8>>, Vec<(&str, Operator, &str, &str)>) {
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
    (wires, gates)
}

fn perform_calculation(wires: &HashMap<&str, Option<u8>>, gates: &Vec<(&str, Operator, &str, &str)>) -> Option<u64> {
    let mut wires = wires.clone();
    let mut none_count = wires.values().filter(|val| val.is_none()).count();
    while none_count > 0 {
        let mut new_none_count = none_count;
        for (input_1, operator, input_2, output) in gates.iter() {
            // Skip if the output wire is already set
            if wires.get(output).unwrap().is_some() {
                continue
            }

            // Use pattern matching to extract input wire values
            let (input_wire_1, input_wire_2) = match (wires.get(input_1), wires.get(input_2)) {
                (Some(Some(input_wire_1)), Some(Some(input_wire_2))) => (input_wire_1, input_wire_2),
                _ => continue,  // Skip if either input is not ready
            };

            // Calculate the output wire value based on the operator
            let output_wire_value = match operator {
                Operator::AND => input_wire_1 & input_wire_2,
                Operator::OR => input_wire_1 | input_wire_2,
                Operator::XOR => input_wire_1 ^ input_wire_2,
            };

            // Set the output wire value and update the none_count
            if let Some(output_wire) = wires.get_mut(output) {
                *output_wire = Some(output_wire_value);
                new_none_count -= 1;
            }
        }
        if new_none_count == none_count {
            return None
        }
        none_count = new_none_count;
    }
    Some(calculate_decimal(&wires, 'z'))
}

/// Lazily generates unique groups of k non-overlapping pairs from the input
pub fn unique_k_pair_combinations<'a>(
    input: &'a [usize],
    k: usize,
) -> impl Iterator<Item = Vec<(usize, usize)>> + 'a {
    input
        .iter()
        .combinations(2) // Generate all pairs lazily
        .combinations(k) // Combine pairs into groups of k lazily
        .filter(move |group| {
            // Ensure no overlaps (unique elements in all pairs)
            let all_elements: Vec<usize> = group
                .iter()
                .flat_map(|pair| vec![*pair[0], *pair[1]]) // Dereference the references to get usize values
                .collect();
            all_elements.iter().unique().count() == k * 2
        })
        .map(|group| {
            // Convert references to owned pairs
            group.into_iter().map(|pair| (*pair[0], *pair[1])).collect()
        })
}

fn calculate_decimal(wires: &HashMap<&str, Option<u8>>, name: char) -> u64 {
    wires.into_iter()
        .filter(|(wire, _)| wire.starts_with(name))
        .sorted_by_key(|kv| kv.0)
        .rev()
        .fold(0, |acc, (_, elem)| acc * 2 + elem.expect("Every wire should have a value by now") as u64)

}

#[test]
fn test_unique_combinations() {
    let input = Vec::from_iter(0..4);
    let result: Vec<Vec<(usize, usize)>> = unique_k_pair_combinations(&input, 2).collect();
    println!("{:?}", result);
    assert_eq!(result.len(), 3)
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
fn input_part_1() {
    let input = include_str!("../input/input_24.txt");
    assert_eq!(part_1(input), 51837135476040)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_24.txt");
    assert_eq!(part_2(input), "hjf,kdh,kpp,sgj,vss,z14,z31,z35")
}
