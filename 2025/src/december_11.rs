use std::collections::HashMap;

pub fn part_1(input: &str) -> u64 {
    let connections: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(to_connections)
        .collect();
    let mut visited = Vec::new();
    find_number_of_paths(&connections, "you", "out", &mut visited)
}

fn to_connections(line: &str) -> (&str, Vec<&str>) {
    let (server, output_str) = line.split_once(':').unwrap();
    let outputs: Vec<&str> = output_str.split_whitespace().collect();
    (server, outputs)
}

fn find_number_of_paths<'a>(
    connections: &HashMap<&str, Vec<&'a str>>,
    current: &'a str,
    target: &str,
    visited: &mut Vec<&'a str>,
) -> u64 {
    if current == target {
        return 1;
    }

    visited.push(current);

    let mut path_count = 0;
    if let Some(neighbors) = connections.get(current) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                path_count += find_number_of_paths(connections, neighbor, target, visited);
            }
        }
    }

    visited.pop();
    path_count
}

pub fn part_2(input: &str) -> u32 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_11.txt");
    assert_eq!(part_1(input), 5)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_11.txt");
    assert_eq!(part_1(input), 683)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_11.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_11.txt");
    assert_eq!(part_2(input), 0)
}
