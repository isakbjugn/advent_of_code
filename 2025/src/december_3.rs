pub fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10).map(u64::from)).collect())
        .map(|bank| get_max_joltage(bank, 2))
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10).map(u64::from)).collect())
        .map(|bank| get_max_joltage(bank, 12))
        .sum()
}

fn get_max_joltage(bank: Vec<u64>, batteries: usize) -> u64 {
    match batteries {
        1 => bank.into_iter().max().expect("Banken er tom"),
        _ => {
            let (max_position, max) = bank[0..bank.len() - (batteries - 1)]
                .iter()
                .enumerate()
                .max_by_key(|&(pos, &value)| (value, std::cmp::Reverse(pos)))
                .expect("Banken er for liten");
            max * 10u64.pow(batteries as u32 - 1) + get_max_joltage(bank[max_position + 1..].to_vec(), batteries - 1)
        }
    }
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
