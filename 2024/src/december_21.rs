use std::collections::HashMap;
use itertools::Itertools;
use crate::with::{With};

pub fn part_1(input: &str) -> u64 {
    let mut memo: HashMap<(char, char, u8), u64> = HashMap::new();
    input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|code| -> u64 {
            let combination_length: u64 = ['A'].with(&code).into_iter()
                .tuple_windows()
                .map(|(from, to)| find_combination_length(from, to, 3, 3, &mut memo))
                .sum();
            combination_length * code_to_number(&code).unwrap() as u64
        })
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    let mut memo: HashMap<(char, char, u8), u64> = HashMap::new();
    input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|code| -> u64 {
            let combination_length: u64 = ['A'].with(&code).into_iter()
                .tuple_windows()
                .map(|(from, to)| find_combination_length(from, to, 26, 26, &mut memo))
                .sum();
            combination_length * code_to_number(&code).unwrap() as u64
        })
        .sum()
}

fn find_combination_length(from: char, to: char, robots: u8, max_robots: u8, memo: &mut HashMap<(char, char, u8), u64>) -> u64 {
    if let Some(&result) = memo.get(&(from, to, robots)) {
        return result
    }
    let result = match robots {
        0 => 1,
        r if r == max_robots => {
            match move_on_num_pad(from, to) {
                (button_sequence, false) => {
                    ['A'].with(&button_sequence).with(&['A']).into_iter()
                        .tuple_windows()
                        .map(|(from, to)| find_combination_length(from, to, robots - 1, max_robots, memo))
                        .sum()
                }
                (button_sequence, true) => {
                    let k = button_sequence.len();
                    button_sequence.into_iter().permutations(k)
                        .map(|permuted_sequence| {
                            ['A'].with(&permuted_sequence).with(&['A']).into_iter()
                                .tuple_windows()
                                .map(|(from, to)| find_combination_length(from, to, robots - 1, max_robots, memo))
                                .sum()
                        })
                        .min().expect("Should be a minimum")
                }
            }
        }
        _ => {
            match move_on_d_pad(from, to) {
                (button_sequence, false) => {
                    ['A'].with(&button_sequence).with(&['A']).into_iter()
                        .tuple_windows()
                        .map(|(from, to)| find_combination_length(from, to, robots - 1, max_robots, memo))
                        .sum()
                }
                (button_sequence, true) => {
                    let k = button_sequence.len();
                    button_sequence.into_iter().permutations(k)
                        .map(|permuted_sequence| {
                            ['A'].with(&permuted_sequence).with(&['A']).into_iter()
                                .tuple_windows()
                                .map(|(from, to)| find_combination_length(from, to, robots - 1, max_robots, memo))
                                .sum()
                        })
                        .min().expect("Should be a minimum")
                }
            }
        }
    };
    memo.insert((from, to, robots), result);
    result
}

fn code_to_number(code: &Vec<char>) -> Option<usize> {
    let numeric_string: String = code
        .into_iter()
        .filter(|c| c.is_ascii_digit())
        .collect();

    // Parse the resulting string into a u64
    numeric_string.parse::<usize>().ok()
}

fn move_on_num_pad(from: char, to: char) -> (Vec<char>, bool) {
    match (from, to) {
        ('A', '0') => (vec!['<'], true),
        ('A', '1') => (vec!['^', '<', '<'], false), // unngå ulovlig posisjon true),
        ('A', '3') => (vec!['^'], true),
        ('A', '4') => (vec!['^', '^', '<', '<'], false), // unngå ulovlig posisjon true),
        ('A', '7') => (vec!['^', '^', '^', '<', '<'], false), // unngå ulovlig posisjon true),
        ('A', '8') => (vec!['<', '^', '^', '^'], true),
        ('A', '9') => (vec!['^', '^', '^'], true),

        ('0', 'A') => (vec!['>'], true),
        ('0', '2') => (vec!['^'], true),

        ('1', '3') => (vec!['>', '>'], true),
        ('1', '7') => (vec!['^', '^'], true),

        ('2', '9') => (vec!['^', '^', '>'], true),

        ('3', 'A') => (vec!['v'], true),
        ('3', '6') => (vec!['^'], true),
        ('3', '7') => (vec!['<', '<', '^', '^'], true),

        ('4', '1') => (vec!['v'], true),
        ('4', '5') => (vec!['>'], true),

        ('5', 'A') => (vec!['v', 'v', '>'], true),
        ('5', '6') => (vec!['>'], true),

        ('6', 'A') => (vec!['v', 'v'], true),

        ('7', '3') => (vec!['v', 'v', '>', '>'], true),
        ('7', '8') => (vec!['>'], true),
        ('7', '9') => (vec!['>', '>'], true),

        ('8', '0') => (vec!['v', 'v', 'v'], true),
        ('8', '3') => (vec!['v', 'v', '>'], true),
        ('8', '5') => (vec!['v'], true),

        ('9', 'A') => (vec!['v', 'v', 'v'], true),
        ('9', '7') => (vec!['<', '<'], true),
        ('9', '8') => (vec!['<'], true),

        _ => unreachable!("from {} to {}", from, to)
    }
}

fn move_on_d_pad(from: char, to: char) -> (Vec<char>, bool) {
    match (from, to) {
        (x, y) if x == y => (Vec::new(), false),

        ('A', '<') => (vec!['v', '<', '<'], false),
        ('A', '^') => (vec!['<'], true),
        ('A', 'v') => (vec!['<', 'v'], true),
        ('A', '>') => (vec!['v'], true),

        ('<', '^') => (vec!['>', '^'], false),
        ('<', 'v') => (vec!['>'], true),
        ('<', '>') => (vec!['>', '>'], true),
        ('<', 'A') => (vec!['>', '>', '^'], false),

        ('v', '<') => (vec!['<'], true),
        ('v', '^') => (vec!['^'], true),
        ('v', '>') => (vec!['>'], true),
        ('v', 'A') => (vec!['>', '^'], true),

        ('^', '<') => (vec!['v', '<'], false),
        ('^', 'v') => (vec!['v'], true),
        ('^', '>') => (vec!['v', '>'], true),
        ('^', 'A') => (vec!['>'], true),

        ('>', '<') => (vec!['<', '<'], true),
        ('>', 'v') => (vec!['<'], true),
        ('>', '^') => (vec!['<', '^'], true),
        ('>', 'A') => (vec!['^'], true),

        _ => {
            unreachable!("from {} to {}", from, to)
        }
    }
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_21.txt");
    assert_eq!(part_1(input), 126384)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_21.txt");
    assert_eq!(part_2(input), 154115708116294)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_21.txt");
    let result = part_1(input);
    println!("Result: {}", result);
    assert_eq!(result, 270084)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_21.txt");
    assert_eq!(part_2(input), 329431019997766)
}
