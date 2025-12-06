use regex::Regex;

pub fn part_1(input: &str) -> u64 {
    let rows: Vec<Vec<&str>> = input
        .lines()
        .map(|row| row.split_whitespace().collect::<Vec<&str>>())
        .collect();
    let problems = rows[0].len();
    let operator_row = rows.len() - 1;
    let mut sum_total = 0u64;

    for problem_index in 0..problems {
        let mut current_result: u64 = rows[0][problem_index].parse().unwrap();
        let operator = rows[operator_row][problem_index];
        for row_index in 1..operator_row {
            let next_value: u64 = rows[row_index][problem_index].parse().unwrap();
            match operator {
                "+" => current_result += next_value,
                "*" => current_result *= next_value,
                _ => panic!("Unknown operator {}", operator),
            }
        }
        sum_total += current_result;
    }
    sum_total
}

pub fn part_2(input: &str) -> u64 {
    let (problem_string, operators_line) = input.trim_end().rsplit_once('\n').unwrap();
    let operators: Vec<&str> = operators_line.split_whitespace().collect();

    let transposed_input = {
        let rows: Vec<&str> = problem_string.lines().collect();
        let mut transposed_rows: Vec<String> = Vec::new();
        let max_length = rows.iter().map(|row| row.len()).max().unwrap_or(0);
        for col_index in 0..max_length {
            let mut new_row = String::new();
            for row in &rows {
                if col_index < row.len() {
                    new_row.push(row.chars().nth(col_index).unwrap());
                } else {
                    new_row.push(' ');
                }
            }
            transposed_rows.push(new_row);
        }
        transposed_rows.join("\n")
    };

    let re = Regex::new(r"\n\s*\n").unwrap();

    re
        .split(transposed_input.as_str())
        .enumerate()
        .map(|(problem_index, problem)| {
            match operators[problem_index] {
                "+" => {
                    problem
                        .lines()
                        .fold(0, |acc, line| acc + line.trim().parse::<u64>().unwrap_or(0))
                }
                "*" => {
                    problem
                        .lines()
                        .fold(1, |acc, line| acc * line.trim().parse::<u64>().unwrap_or(1))
                },
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
