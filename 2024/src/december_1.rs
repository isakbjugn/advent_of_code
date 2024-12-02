use itertools::Itertools;

pub fn part_1(input: &str) -> u32 {
    let (mut first_list, mut second_list): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .unzip();
    first_list.sort();
    second_list.sort();

    first_list
        .iter()
        .zip(second_list.iter())
        .map(|(a, &b)| a.abs_diff(b))
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    let (first_list, second_list): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .unzip();

    let counts = second_list.into_iter().counts();

    first_list
        .iter()
        .map(|a| a * counts.get(a).map(|&b| b as u32).unwrap_or(0))
        .sum()
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_1.txt");
    assert_eq!(part_1(input), 11)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_1.txt");
    assert_eq!(part_2(input), 31)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_1.txt");
    assert_eq!(part_1(input), 765748)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_1.txt");
    assert_eq!(part_2(input), 27732508)
}
