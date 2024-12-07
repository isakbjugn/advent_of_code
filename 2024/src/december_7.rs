use itertools::{repeat_n, Itertools};
use rayon::prelude::*;

pub fn part_1(input: &str) -> u64 {
    to_ans_and_operands(input)
        .filter(|(ans, operands)| can_be_created(ans, operands, &['+', '*']))
        .map(|(ans, _)| ans)
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    to_ans_and_operands(input)
        .filter(|(ans, operands)| can_be_created(ans, operands, &['+', '*', '|']))
        .map(|(ans, _)| ans)
        .sum()
}

fn to_ans_and_operands(input: & str) -> impl ParallelIterator<Item = (u64, Vec<u64>)> + use<'_> {
    input.par_lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(first, second)| (
            first.parse::<u64>().expect("Could not create ans value"),
            second.split(" ")
                .map(|string| string.parse::<u64>().expect("Could not create operand values"))
                .collect::<Vec<u64>>())
        )
}

fn can_be_created(ans: &u64, operands: &[u64], basis: &[char]) -> bool {
    let mut operator_sequences = repeat_n(basis, operands.len() - 1).multi_cartesian_product();

    operator_sequences.any(|operator_sequence| {
        let mut product_sum = *operands.first().unwrap();
        for (index, operand) in operands.iter().skip(1).enumerate() {
            match operator_sequence.get(index) {
                Some('+') => product_sum += operand,
                Some('*') => product_sum *= operand,
                Some('|') => product_sum = concat(product_sum, *operand),
                _ => unreachable!()
            }
        }
        product_sum == *ans
    })
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_7.txt");
    assert_eq!(part_1(input), 3749)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_7.txt");
    assert_eq!(part_2(input), 11387)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_7.txt");
    assert_eq!(part_1(input), 21572148763543)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_7.txt");
    assert_eq!(part_2(input), 581941094529163)
}
