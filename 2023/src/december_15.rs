use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use memoize::memoize;
use crate::sum_to::SumTo;

pub fn part_1(input: &str) -> u32 {
    input.split(',').map(hash).sum_to()
}

fn hash(string: &str) -> u8 {
    let mut value = 0;
    for char in string.chars() {
        value = normalise(value as u16 + char as u16);
    }
    value
}

#[memoize]
fn normalise(value: u16) -> u8 {
    ((value * 17) % 256) as u8
}

pub fn part_2(input: &str) -> u32 {
    let mut hashmap: HashMap<u8, Vec<(&str, u8)>> = HashMap::new();
    for string in input.split(',') {
        match string.contains('=') {
            true => {
                let (label, focal_length_str) = string.split_once('=').unwrap();
                let lens_box = hash(label);
                let focal_length = focal_length_str.parse().unwrap();
                if let Vacant(e) = hashmap.entry(lens_box) {
                    e.insert(vec![(label, focal_length)]);
                } else {
                    match hashmap.get(&lens_box).unwrap().iter().position(|&(l, _)| l == label) {
                        Some(idx) => hashmap.get_mut(&lens_box).unwrap()[idx] = (label, focal_length),
                        None => hashmap.get_mut(&lens_box).unwrap().push((label, focal_length))
                    }
                }
            }
            false => {
                let (label, _) = string.split_once('-').unwrap();
                let lens_box = hash(label);
                if hashmap.contains_key(&lens_box) {
                    if let Some(idx) = hashmap.get(&lens_box).unwrap().iter().position(|&(l, _)| l == label) {
                        hashmap.get_mut(&lens_box).unwrap().remove(idx);
                    }
                }
            }
        }
    }
    hashmap.iter().map(|(&box_idx, box_content)|
        box_content.iter().enumerate()
            .map(|(slot, &(_, focal_length))|
                focusing_power(box_idx, slot, focal_length)
            )
            .sum::<u16>()
    ).sum_to()
}

fn focusing_power(box_number: u8, slot: usize, focal_length: u8) -> u16 {
    (box_number as u16 + 1) * (slot as u16 + 1) * focal_length as u16
}

#[test]
fn sample_input_part_1_0() {
    let input = "HASH";
    assert_eq!(part_1(input), 52)
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_15.txt");
    assert_eq!(part_1(input), 1320)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_15.txt");
    assert_eq!(part_2(input), 145)
}