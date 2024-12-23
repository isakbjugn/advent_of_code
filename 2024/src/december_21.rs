use std::collections::HashMap;
use itertools::Itertools;
use crate::with::{With};

pub fn part_1(input: &str) -> u64 {
    sum_complexities(input, 2)
}

pub fn part_2(input: &str) -> u64 {
    sum_complexities(input, 25)
}

fn sum_complexities(input: &str, robots: u8) -> u64 {
    let mut memo: HashMap<(char, char, u8), u64> = HashMap::new();
    input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|code| -> u64 {
            let combination_length: u64 = ['A'].with(&code).into_iter()
                .tuple_windows()
                .map(|(from, to)| find_combination_length(from, to, robots, robots, &mut memo))
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
        0 => {
            ['A'].with(&move_on_d_pad(from, to).0).len() as u64
        }
        r => {
            let get_moves = if r == max_robots { move_on_num_pad } else { move_on_d_pad };
            moves_with_all_variants(from, to, get_moves).into_iter()
                .map(|sequence| {
                    sequence.into_iter()
                        .tuple_windows()
                        .map(|(from, to)| find_combination_length(from, to, robots - 1, max_robots, memo))
                        .sum()
                })
                .min().expect("Should be a minimum")
        }
    };
    memo.insert((from, to, robots), result);
    result
}

fn code_to_number(code: &[char]) -> Option<usize> {
    let numeric_string: String = code
        .iter()
        .filter(|c| c.is_ascii_digit())
        .collect();

    numeric_string.parse::<usize>().ok()
}

fn moves_with_all_variants(from: char, to: char, get_moves: fn(char, char) -> (Vec<char>, bool)) -> Vec<Vec<char>> {
    match get_moves(from, to) {
        (sequence, false) => vec![['A'].with(&sequence).with(&['A'])],
        (sequence, true) => {
            let k = sequence.len();
            sequence.into_iter().permutations(k).map(|permuted_sequence| ['A'].with(&permuted_sequence).with(&['A'])).collect()
        }
    }
}

fn move_on_num_pad(from: char, to: char) -> (Vec<char>, bool) {
    match (from, to) {
        ('A', '0') => (vec!['<'], true),
        ('A', '1') => (vec!['^', '<', '<'], false),
        ('A', '3') => (vec!['^'], true),
        ('A', '4') => (vec!['^', '^', '<', '<'], false),
        ('A', '7') => (vec!['^', '^', '^', '<', '<'], false),
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
    assert_eq!(part_1(input), 270084)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_21.txt");
    assert_eq!(part_2(input), 329431019997766)
}
