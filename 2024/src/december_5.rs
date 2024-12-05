use std::collections::HashMap;
use itertools::Itertools;

pub fn part_1(input: &str) -> u32 {
    let (rules_str, updates_str) = input.split_once("\n\n").expect("Wrong input format!");
    let rules = rules_str
        .lines()
        .map(|line| line.split_once('|').expect("Rules not formatted as 1|2"))
        .map(|(one, two)| (one.parse::<u32>().unwrap(), two.parse::<u32>().unwrap()))
        .collect::<Vec<(u32, u32)>>();

    let updates = updates_str
        .lines()
        .map(|line| line.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>() )
        .collect::<Vec<Vec<u32>>>();
    
    updates.into_iter().filter(|update| {
        for (index, page) in update.iter().enumerate() {
            if rules.iter().filter(|(one, _)| one == page)
                .any(|(_, later_page)| {
                    if let Some((later_index, _)) = update.iter().find_position(|&p| p == later_page ) {
                        return if later_index < index { true } else { false }
                    }
                    false
                }) {
                return false
            } else {
                continue
            }
        }
        true
    })
        .map(|update| {
            let middle_index = (update.len() - 1) / 2;
            *update.get(middle_index).expect("No middle element!")
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_5.txt");
    assert_eq!(part_1(input), 143)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_5.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_5.txt");
    assert_eq!(part_1(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_5.txt");
    assert_eq!(part_2(input), 0)
}
