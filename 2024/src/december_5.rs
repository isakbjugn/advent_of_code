use std::cmp::Ordering;

pub fn part_1(input: &str) -> u32 {
    let (rules, updates) = read_input(input);

    updates
        .into_iter()
        .filter(|update| {
            update.is_sorted_by(|a, b| sort_by_rules(a, b, &rules) != Ordering::Greater)
        })
        .map(middle_element)
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    let (rules, updates) = read_input(input);

    updates
        .into_iter()
        .filter(|update| {
            !update.is_sorted_by(|a, b| sort_by_rules(a, b, &rules) != Ordering::Greater)
        })
        .map(|mut update| {
            update.sort_by(|a, b| sort_by_rules(a, b, &rules));
            update
        })
        .map(middle_element)
        .sum()
}

fn read_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules_str, updates_str) = input.split_once("\n\n").expect("Wrong input format!");
    let rules = rules_str
        .lines()
        .map(|line| line.split_once('|').expect("Rules not formatted as 1|2"))
        .map(|(one, two)| (one.parse::<u32>().unwrap(), two.parse::<u32>().unwrap()))
        .collect::<Vec<(u32, u32)>>();

    let updates = updates_str
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    (rules, updates)
}

fn sort_by_rules(a: &u32, b: &u32, rules: &[(u32, u32)]) -> Ordering {
    rules
        .iter()
        .find_map(|(one, two)| {
            if one == a && two == b {
                Some(Ordering::Less)
            } else if one == b && two == a {
                Some(Ordering::Greater)
            } else {
                None
            }
        })
        .unwrap_or(Ordering::Equal)
}

fn middle_element(vec: Vec<u32>) -> u32 {
    let middle_index = (vec.len() - 1) / 2;
    *vec.get(middle_index).expect("No middle element!")
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_5.txt");
    assert_eq!(part_1(input), 143)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_5.txt");
    assert_eq!(part_2(input), 123)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_5.txt");
    assert_eq!(part_1(input), 6612)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_5.txt");
    assert_eq!(part_2(input), 4944)
}
