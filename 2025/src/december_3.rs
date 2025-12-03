pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(get_max_joltage_with_two_batteries)
        .sum()
}

fn get_max_joltage_with_two_batteries(bank: &str) -> u32 {
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
    input
        .lines()
        .map(get_largest_twelve_digit_number_from_string)
        .sum()
}

fn get_largest_twelve_digit_number_from_string(bank: &str) -> u64 {
    let digits = bank.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
    let mut selection = digits[0..12].to_vec();
    for position in 12..digits.len() {
        // if removing the smallest digit and adding the new digit results in a larger number, do it
        let next_digit = digits[position];
        let mut max = selection.iter().fold(0u64, |acc, &d| acc * 10 + d as u64);
        let mut remove_position = None;
        for new_position in 0..12 {
            let mut temp_selection = selection.clone();
            temp_selection.remove(new_position);
            temp_selection.push(next_digit);
            let sum = temp_selection.iter().fold(0u64, |acc, &d| acc * 10 + d as u64);
            if sum > max {
                remove_position = Some(new_position);
                max = sum;
            }
        }
        if let Some(pos) = remove_position {
            selection.remove(pos);
            selection.push(next_digit);
        }
    }
    let number =
        selection.iter().fold(0u64, |acc, &d| acc * 10 + d as u64);
    number
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
    assert_eq!(part_2(input), 3121910778619)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_3.txt");
    assert_eq!(part_2(input), 175053592950232)
}
