use memoize::memoize;

pub fn part_1(input: &str) -> u32 {
    input.split(',').map(hash).map(|h| h as u32).sum()
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

    0
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
    assert_eq!(part_2(input), 0)
}