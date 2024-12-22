use rayon::prelude::*;

pub fn part_1(input: &str) -> u64 {
    input.par_lines()
        .map(|line| line.parse::<u64>().expect("Secret number must parse to int"))
        .map(|secret_number| {
            let mut number = secret_number;
            for _ in 0..2000 {
                number = generate(number);
            }
            // println!("{}: {}", secret_number, number);
            number
        })
        .sum()
}

fn generate(number: u64) -> u64 {
    let first_mixer = number * 64;
    let number = mix_and_prune(number, first_mixer);
    let second_mixer = number / 32;
    let number = mix_and_prune(number, second_mixer);
    let third_mixer = number * 2048;
    mix_and_prune(number, third_mixer)
}

fn mix_and_prune(number: u64, mixer: u64) -> u64 {
    (number ^ mixer) % 16777216
}

pub fn part_2(input: &str) -> usize {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_22.txt");
    assert_eq!(part_1(input), 37327623)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_22.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_22.txt");
    assert_eq!(part_1(input), 17163502021)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_22.txt");
    assert_eq!(part_2(input), 0)
}
