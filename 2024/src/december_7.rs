use itertools::Itertools;

pub fn part_1(input: &str) -> u64 {
    to_ans_and_operands(input)
        .filter(|(ans, operands)| can_be_created(ans, operands, vec!['+', '*']))
        .map(|(ans, _)| ans)
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    to_ans_and_operands(input)
        .filter(|(ans, operands)| can_be_created(ans, operands, vec!['+', '*', '|']))
        .map(|(ans, _)| ans)
        .sum()
}

fn to_ans_and_operands(input: & str) -> impl Iterator<Item = (u64, Vec<u64>)> + use<'_> {
    input.lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(first, second)| (
            first.parse::<u64>().expect("Could not create ans value"),
            second.split(" ")
                .map(|string| string.parse::<u64>().expect("Could not create operand values"))
                .collect::<Vec<u64>>())
        )
}

fn can_be_created(ans: &u64, operands: &[u64], basis: Vec<char>) -> bool {
    let operator_sequences = basis.permutations_with_replacement(operands.len() - 1);

    for operator_sequence in operator_sequences {
        let mut product_sum = *operands.first().unwrap();
        for (index, operand) in operands.iter().skip(1).enumerate() {
            match operator_sequence.get(index) {
                Some('+') => product_sum += operand,
                Some('*') => product_sum *= operand,
                Some('|') => product_sum = concat(product_sum, *operand),
                _ => unreachable!()
            }
        }
        if product_sum == *ans {
            return true
        }
    }
    false
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

trait PermutationsWithReplacement {
    fn permutations_with_replacement(&self, k: usize) -> Vec<Self> where Self: Sized;
}

impl PermutationsWithReplacement for Vec<char> {
    fn permutations_with_replacement(&self, k: usize) -> Vec<Self> {
        (0..=k - 1).map(|_| 0..=self.len() - 1)
            .multi_cartesian_product()
            .map(|permutation|
                permutation
                    .into_iter()
                    .map(|index| self.get(index).expect("Could not access basis"))
                    .cloned()
                    .collect()
            )
            .collect::<Vec<Vec<char>>>()
    }
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
