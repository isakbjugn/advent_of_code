use std::collections::HashSet;

pub fn part_1(input: &str) -> u64 {
    let boxes: Vec<(u32, u32, u32)> = input.lines().map(to_box).collect();
    let connections = if boxes.len() > 20 { 1000 } else { 10 };
    let closest_box_pairs = get_n_closest_box_pairs(&boxes, Some(connections));
    let mut circuits: Vec<HashSet<u32>> = Vec::new();

    for box_pair in closest_box_pairs {
        let mut touched_circuits = 0;
        for circuit in &mut circuits {
            if circuit.contains(&box_pair.0) || circuit.contains(&box_pair.1) {
                circuit.insert(box_pair.0);
                circuit.insert(box_pair.1);
                touched_circuits += 1;
            }
        }
        if touched_circuits > 1 {
            let mut merged_circuit: HashSet<u32> = HashSet::new();
            circuits.retain(|circuit| {
                if circuit.contains(&box_pair.0) || circuit.contains(&box_pair.1) {
                    merged_circuit.extend(circuit);
                    false
                } else {
                    true
                }
            });
            circuits.push(merged_circuit);
        } else if touched_circuits == 0 {
            circuits.push(HashSet::from([box_pair.0, box_pair.1]));
        }
    }

    get_product_of_three_largest_circuits(&circuits)
}

fn to_box(box_str: &str) -> (u32, u32, u32) {
    let box_vec: Vec<u32> = box_str.split(',').map(|c| c.parse().unwrap()).collect();
    (box_vec[0], box_vec[1], box_vec[2])
}

fn get_n_closest_box_pairs(boxes: &[(u32, u32, u32)], n: Option<usize>) -> Vec<(u32, u32)> {
    let mut box_pairs: Vec<(u32, u32, f64)> = Vec::new();

    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let distance = euclidean_distance(boxes[i], boxes[j]);
            box_pairs.push((i as u32, j as u32, distance));
        }
    }

    box_pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    match n {
        None => box_pairs.into_iter().map(|(b1, b2, _)| (b1, b2)).collect(),
        Some(n) => box_pairs.into_iter().take(n).map(|(b1, b2, _)| (b1, b2)).collect(),
    }
}

fn euclidean_distance(p0: (u32, u32, u32), p1: (u32, u32, u32)) -> f64 {
    let dx = (p0.0 as f64) - (p1.0 as f64);
    let dy = (p0.1 as f64) - (p1.1 as f64);
    let dz = (p0.2 as f64) - (p1.2 as f64);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn get_product_of_three_largest_circuits(circuits: &[HashSet<u32>]) -> u64 {
    let mut circuit_sizes: Vec<usize> = circuits.iter().map(|circuit| circuit.len()).collect();
    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));
    circuit_sizes.iter().take(3).map(|&size| size as u64).product()
}

pub fn part_2(input: &str) -> u64 {
    let boxes: Vec<(u32, u32, u32)> = input.lines().map(to_box).collect();
    let sorted_box_pairs = get_n_closest_box_pairs(&boxes, None);
    let mut circuits: Vec<HashSet<u32>> = Vec::new();
    let mut completing_pair: Option<(u32, u32)> = None;

    for box_pair in sorted_box_pairs.iter() {
        let mut touched_circuits = 0;
        for circuit in &mut circuits {
            if circuit.contains(&box_pair.0) || circuit.contains(&box_pair.1) {
                circuit.insert(box_pair.0);
                circuit.insert(box_pair.1);
                touched_circuits += 1;
                if circuit.len() == boxes.len() && completing_pair.is_none() {
                    completing_pair = Some(*box_pair);
                    break
                }
            }
        }
        if touched_circuits > 1 {
            let mut merged_circuit: HashSet<u32> = HashSet::new();
            circuits.retain(|circuit| {
                if circuit.contains(&box_pair.0) || circuit.contains(&box_pair.1) {
                    merged_circuit.extend(circuit);
                    false
                } else {
                    true
                }
            });
            if merged_circuit.len() == boxes.len() && completing_pair.is_none() {
                completing_pair = Some(*box_pair);
                break
            }
            circuits.push(merged_circuit);
        } else if touched_circuits == 0 {
            circuits.push(HashSet::from([box_pair.0, box_pair.1]));
        }
    }

    if let Some((box_1, box_2)) = completing_pair {
        boxes[box_1 as usize].0 as u64 * boxes[box_2 as usize].0 as u64
    } else {
        let last_pair = sorted_box_pairs.last().unwrap();
        boxes[last_pair.0 as usize].0 as u64 * boxes[last_pair.1 as usize].0 as u64
    }
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_8.txt");
    assert_eq!(part_1(input), 40)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_8.txt");
    assert_eq!(part_1(input), 97384)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_8.txt");
    assert_eq!(part_2(input), 25272)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_8.txt");
    assert_eq!(part_2(input), 9003685096)
}
