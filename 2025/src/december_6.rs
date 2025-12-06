use regex::Regex;
use crate::transpose::Transpose;

pub fn part_1(input: &str) -> u64 {
    let (problem_string, operators_line) = input.trim_end().rsplit_once('\n').unwrap();
    let problems: Vec<Vec<u64>> = problem_string
        .lines()
        .map(|row| row.split_whitespace().map(|num| num.parse::<u64>().unwrap()).collect())
        .collect();
    let transposed_problems = problems.transpose();
    let operators: Vec<&str> = operators_line.split_whitespace().collect();

    calculate(&transposed_problems, &operators)
}

pub fn part_2(input: &str) -> u64 {
    let (problem_string, operators_line) = input.trim_end().rsplit_once('\n').unwrap();
    let transposed_problem_string = String::from(problem_string).transpose();
    let re = Regex::new(r"\n\s*\n").unwrap();
    let transposed_problems: Vec<Vec<u64>> = re.split(&transposed_problem_string)
        .map(|row| row.split_whitespace().map(|num| num.parse::<u64>().unwrap()).collect())
        .collect();
    let operators: Vec<&str> = operators_line.split_whitespace().collect();

    calculate(&transposed_problems, &operators)
}

fn calculate(problems: &[Vec<u64>], operators: &[&str]) -> u64 {
    problems
        .iter()
        .enumerate()
        .map(|(problem_index, problem)| {
            match operators[problem_index] {
                "+" => problem.iter().sum::<u64>(),
                "*" => problem.iter().product::<u64>(),
                &_ => panic!("Unknown operator {}", operators[problem_index]),
            }
        })
        .sum()
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_6.txt");
    assert_eq!(part_1(input), 4277556)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_6.txt");
    assert_eq!(part_1(input), 3968933219902)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_6.txt");
    assert_eq!(part_2(input), 3263827)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_6.txt");
    assert_eq!(part_2(input), 6019576291014)
}
