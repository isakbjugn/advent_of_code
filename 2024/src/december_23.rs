use std::collections::{HashMap, HashSet};

pub fn part_1(input: &str) -> usize {
    let pairs: Vec<(&str, &str)> = input.lines()
        .map(|line| line.split_once('-').expect("Connection has hyphen")).collect();
    let mut connections_per_computer: HashMap<&str, Vec<&str>> = HashMap::new();
    pairs.iter().for_each(|(first, second)| {
        connections_per_computer.entry(first).or_insert(Vec::new()).push(second);
        connections_per_computer.entry(second).or_insert(Vec::new()).push(first);
    });
    
    let mut triples: HashSet<(&str, &str, &str)> = HashSet::new();
    pairs.iter().for_each(|(first, second)| {
        for connection in connections_per_computer.get(first).expect("First computer has connections") {
            if connections_per_computer.get(second).expect("Certainly second computer has also!").contains(connection) {
                triples.insert(canonicalize((first, second, connection)));
            }
        }
    });
    
    let triples_starting_with_t: HashSet<(&str, &str, &str)> = triples.into_iter().filter(|(first, second, third)| {
        first.starts_with('t') || second.starts_with('t') || third.starts_with('t')
    }).collect();

    triples_starting_with_t.len()
}

fn canonicalize<'a>((a, b, c): (&'a str, &'a str, &'a str)) -> (&'a str, &'a str, &'a str) {
    let mut v = [a, b, c];
    v.sort();
    (v[0], v[1], v[2])
}

pub fn part_2(input: &str) -> u32 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_23.txt");
    assert_eq!(part_1(input), 7)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_23.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_23.txt");
    assert_eq!(part_1(input), 1485)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_23.txt");
    assert_eq!(part_2(input), 0)
}
