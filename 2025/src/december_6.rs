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
    0
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
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_6.txt");
    assert_eq!(part_2(input), 0)
}
