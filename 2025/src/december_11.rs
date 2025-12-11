use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn part_1(input: &str) -> u64 {
    let connections: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(to_connections)
        .collect();
    find_number_of_paths(&connections, "you", "out", &mut HashSet::new())
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
    visited: &mut HashSet<&'a str>,
) -> u64 {
    if current == target {
        return 1;
    }

    let mut visited_here = visited.clone();
    visited_here.insert(current);

    match connections.get(current) {
        None => 0,
        Some(neighbors) => neighbors
            .par_iter()
            .filter(|&&neighbor| !visited_here.contains(&neighbor))
            .map(|&neighbor| {
                let mut child_visited = visited_here.clone();
                find_number_of_paths(connections, neighbor, target, &mut child_visited)
            })
            .sum(),
    }
}

pub fn part_2(input: &str) -> u64 {
    let mut connections: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(to_connections)
        .collect();

    let num_paths_from_svr_to_fft = find_number_of_backwards_paths(&connections, "svr", "fft", &mut HashSet::new()); // 7180
    let paths_from_dac_to_out = find_all_paths(&connections, "dac", "out", &mut HashSet::new());
    let num_paths_from_dac_to_out = paths_from_dac_to_out.len(); // 13391
    let servers_in_path_from_dac_to_out: HashSet<&str> = paths_from_dac_to_out
        .into_iter()
        .flat_map(|path| path.into_iter())
        .collect();
    connections.retain(|server, _| !servers_in_path_from_dac_to_out.contains(server) );
    let num_paths_from_fft_to_dac = find_number_of_paths(&connections, "fft", "dac", &mut HashSet::new()); // 5553940
    num_paths_from_svr_to_fft * num_paths_from_fft_to_dac * num_paths_from_dac_to_out as u64
}

fn find_number_of_backwards_paths<'a>(
    connections: &HashMap<&str, Vec<&'a str>>,
    current: &'a str,
    target: &str,
    visited: &mut HashSet<&'a str>,
) -> u64 {
    if current == target {
        return 1;
    }

    let mut visited_here = visited.clone();
    visited_here.insert(current);

    connections
        .par_iter()
        .filter(|(_, outputs)| outputs.contains(&target))
        .map(|(server, _)| {
            let mut child_visited = visited_here.clone();
            find_number_of_backwards_paths(connections, current, server, &mut child_visited)
        })
        .sum()
}

fn find_all_paths<'a>(
    connections: &HashMap<&str, Vec<&'a str>>,
    current: &'a str,
    target: &'a str,
    visited: &mut HashSet<&'a str>,
) -> Vec<Vec<&'a str>> {
    if current == target {
        return vec![vec![target]];
    }

    visited.insert(current);

    let mut paths = Vec::new();
    if let Some(neighbors) = connections.get(current) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                let child_paths = find_all_paths(connections, neighbor, target, visited);
                for mut path in child_paths {
                    let mut full_path = vec![current];
                    full_path.append(&mut path);
                    paths.push(full_path);
                }
            }
        }
    }

    visited.remove(current);
    paths
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
    let input = include_str!("../input/sample_11_2.txt");
    assert_eq!(part_2(input), 2)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_11.txt");
    assert_eq!(part_2(input), 533996779677200)
}
