use itertools::Itertools;

pub fn part_1(input: &str) -> u64 {
    input.lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(first, second)| (
            first.parse::<u64>().expect("Could not create ans value"),
            second.split(" ")
                .map(|string| string.parse::<u64>().expect("Could not create operand values"))
                .collect::<Vec<u64>>())
        )
        .filter(can_be_created)
        .map(|(ans, _)| ans)
        .sum()
}

fn can_be_created(input: &(u64, Vec<u64>)) -> bool {
    let (ans, operands) = input;
    let operator_sequences = (1..=operands.len() - 1).map(|_| 1..=2).multi_cartesian_product()
        .collect::<Vec<Vec<u64>>>();

    // println!("Operator sequences: {:?}", operator_sequences);
    for operator_sequence in operator_sequences {
        let mut product_sum = *operands.first().unwrap();
        // println!("Starting with {product_sum}");
        for (index, operand) in operands.iter().skip(1).enumerate() {
            let operator = match operator_sequence.get(index) {
                Some(1) => '+',
                Some(2) => '*',
                _ => unreachable!()
            };
            // println!("Operation: {product_sum} {operator} {operand}");
            match operator_sequence.get(index) {
                Some(1) => product_sum += operand,
                Some(2) => product_sum *= operand,
                _ => unreachable!()
            }
            // println!("Result: {product_sum}");
        }
        // println!("Comparing {product_sum} and {ans}");
        if product_sum == *ans {
            return true
        }
    }
    false
}

pub fn part_2(input: &str) -> u64 {
    input.lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(first, second)| (
            first.parse::<u64>().expect("Could not create ans value"),
            second.split(" ")
                .map(|string| string.parse::<u64>().expect("Could not create operand values"))
                .collect::<Vec<u64>>())
        )
        .filter(can_be_created_2)
        .map(|(ans, _)| ans)
        .sum()
}

fn can_be_created_2(input: &(u64, Vec<u64>)) -> bool {
    let (ans, operands) = input;
    let operator_sequences = (1..=operands.len() - 1).map(|_| 1..=3).multi_cartesian_product()
        .collect::<Vec<Vec<u64>>>();

    // println!("Operator sequences: {:?}", operator_sequences);
    for operator_sequence in operator_sequences {
        let mut product_sum = *operands.first().unwrap();
        // println!("Starting with {product_sum}");
        for (index, operand) in operands.iter().skip(1).enumerate() {
            let operator = match operator_sequence.get(index) {
                Some(1) => "+",
                Some(2) => "*",
                Some(3) => "||",
                _ => unreachable!()
            };
            // println!("Operation: {product_sum} {operator} {operand}");
            match operator_sequence.get(index) {
                Some(1) => product_sum += operand,
                Some(2) => product_sum *= operand,
                Some(3) => product_sum = concat(product_sum, *operand),
                _ => unreachable!()
            }
            // println!("Result: {product_sum}");
        }
        // println!("Comparing {product_sum} and {ans}");
        if product_sum == *ans {
            return true
        }
    }
    false
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
