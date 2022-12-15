use std::collections::HashSet;

pub fn part_1(input: &str) -> u32 {
    let mut rucksack: (&str, &str);
    let mut shared_char: char;
    let mut sum = 0;

    for line in input.lines() {
        rucksack = line.split_at(line.chars().count()/2);
        shared_char = common_char(rucksack.0, rucksack.1);
        sum += char_to_int(shared_char);
    }
    sum
}

fn common_char(s1: &str, s2: &str) -> char {
    let set1 : HashSet<char> = s1.chars().collect();
    let set2 : HashSet<char> = s2.chars().collect();
    *set1.intersection(&set2).next().unwrap()
}

fn char_to_int(c: char) -> u32 {
    match c.is_ascii_uppercase() {
        true => c as u32 - 38,
        false => c as u32 - 96,
    }
}

pub fn part_2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut shared_char: char;
    let mut sum = 0;

    for group in lines.chunks(3) {
        shared_char = common_chars(group[0], group[1], group[2]);
        sum += char_to_int(shared_char)
    }
    sum
}

fn common_chars(s1: &str, s2: &str, s3: &str) -> char {
    let set1 : HashSet<char> = s1.chars().collect();
    let set2 : HashSet<char> = s2.chars().collect();
    let set3 : HashSet<char> = s3.chars().collect();
    *set1.intersection(&set2).copied().collect::<HashSet<char>>().intersection(&set3).next().unwrap()
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_3.txt");
    assert_eq!(part_1(input), 157)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_3.txt");
    assert_eq!(part_2(input), 70)
}
