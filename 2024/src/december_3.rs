use itertools::Itertools;
use regex::{Captures, Regex};
use std::collections::HashMap;

pub fn part_1(input: &str) -> u32 {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(multiply_captures)
        .sum()
}

enum Instruction {
    Mul(u32),
    Do,
    Dont,
}

fn multiply_captures(captures: Captures) -> u32 {
    let num1 = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let num2 = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    num1 * num2
}

pub fn part_2(input: &str) -> u32 {
    let mut instructions = HashMap::<usize, Instruction>::new();

    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|captures| {
            (
                captures.get(0).unwrap().start(),
                multiply_captures(captures),
            )
        })
        .for_each(|(index, product)| {
            instructions.insert(index, Instruction::Mul(product));
        });

    Regex::new(r"do\(\)")
        .unwrap()
        .captures_iter(input)
        .map(|captures| captures.get(0).unwrap().start())
        .for_each(|index| {
            instructions.insert(index, Instruction::Do);
        });

    Regex::new(r"don't\(\)")
        .unwrap()
        .captures_iter(input)
        .map(|captures| captures.get(0).unwrap().start())
        .for_each(|index| {
            instructions.insert(index, Instruction::Dont);
        });

    let mut do_multiply = true;
    let mut sum = 0;
    for (_, instruction) in instructions.into_iter().sorted_by_key(|(index, _)| *index) {
        match instruction {
            Instruction::Mul(product) if do_multiply => sum += product,
            Instruction::Do => do_multiply = true,
            Instruction::Dont => do_multiply = false,
            _ => continue,
        }
    }
    sum
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_3_1.txt");
    assert_eq!(part_1(input), 161)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_3_2.txt");
    assert_eq!(part_2(input), 48)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_3.txt");
    assert_eq!(part_1(input), 173731097)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_3.txt");
    assert_eq!(part_2(input), 93729253)
}
