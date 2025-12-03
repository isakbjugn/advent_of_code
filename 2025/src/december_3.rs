pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(get_max_joltage)
        .sum()
}

fn get_max_joltage(bank: &str) -> u32 {
    let first = 0;
    for first_digit in (1..=9).rev() {
        // find first occurrence of digit in bank
        if let Some(pos) = bank.find(char::from_digit(first_digit, 10).unwrap()) {
            if let Some(second_digit) = bank[pos + 1..]
                .chars()
                .filter_map(|c| c.to_digit(10))
                .max() {
                return first_digit * 10 + second_digit;
            }
        }
    }
    first
}

pub fn part_2(input: &str) -> u64 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_3.txt");
    assert_eq!(part_1(input), 357)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_3.txt");
    assert_eq!(part_1(input), 17554)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_3.txt");
    //assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_3.txt");
    //assert_eq!(part_2(input), 0)
}
