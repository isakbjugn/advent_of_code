pub fn part_1(input: &str) -> u64 {
    let boxes: Vec<(u32, u32, u32)> = input.lines().map(to_box).collect();
    let connections = if boxes.len() > 20 { 1000 } else { 10 };
    let closest_box_pairs = n_closest_box_pairs(&boxes, connections);
    let mut circuits: Vec<Vec<u32>> = Vec::new();

    for box_pair in closest_box_pairs {
        let mut added_once = false;
        let mut added_multiple = false;
        for circuit in &mut circuits {
            if circuit.contains(&box_pair.0) || circuit.contains(&box_pair.1) {
                if !circuit.contains(&box_pair.0) {
                    circuit.push(box_pair.0);
                }
                if !circuit.contains(&box_pair.1) {
                    circuit.push(box_pair.1);
                }
                if added_once {
                    added_multiple = true;
                } else {
                    added_once = true;
                }
            }
        }
        if added_multiple {
            let mut merged_circuit: Vec<u32> = Vec::new();
            circuits.retain(|circuit| {
                if circuit.contains(&box_pair.0) || circuit.contains(&box_pair.1) {
                    for &b in circuit {
                        if !merged_circuit.contains(&b) {
                            merged_circuit.push(b);
                        }
                    }
                    false
                } else {
                    true
                }
            });
            circuits.push(merged_circuit);
        } else if !added_once {
            circuits.push(vec![box_pair.0, box_pair.1]);
        }
    }

    get_product_of_three_largest_circuits(&circuits)
}

fn to_box(box_str: &str) -> (u32, u32, u32) {
    let box_vec: Vec<u32> = box_str.split(',').map(|c| c.parse().unwrap()).collect();
    (box_vec[0], box_vec[1], box_vec[2])
}

fn n_closest_box_pairs(boxes: &[(u32, u32, u32)], n: usize) -> Vec<(u32, u32)> {
    let mut box_pairs: Vec<(u32, u32, f64)> = Vec::new();

    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let distance = euclidean_distance(boxes[i], boxes[j]);
            box_pairs.push((i as u32, j as u32, distance));
        }
    }

    box_pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    box_pairs.into_iter().take(n).map(|(b1, b2, _)| (b1, b2)).collect()
}

fn euclidean_distance(p0: (u32, u32, u32), p1: (u32, u32, u32)) -> f64 {
    let dx = (p0.0 as f64) - (p1.0 as f64);
    let dy = (p0.1 as f64) - (p1.1 as f64);
    let dz = (p0.2 as f64) - (p1.2 as f64);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn get_product_of_three_largest_circuits(circuits: &[Vec<u32>]) -> u64 {
    let mut circuit_sizes: Vec<usize> = circuits.iter().map(|circuit| circuit.len()).collect();
    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));
    circuit_sizes.iter().take(3).map(|&size| size as u64).product()
}

pub fn part_2(input: &str) -> u64 {
    0
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
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_8.txt");
    assert_eq!(part_2(input), 0)
}
