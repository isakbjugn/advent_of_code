pub fn part_1(input: &str) -> i64 {
    input.lines()
        .map(|line| line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .map(|sequence| sequence.last().unwrap() + extrapolate(&sequence)).sum()
}

fn extrapolate(sequence: &[i64]) -> i64 {
    let differences = differences(sequence);
    if differences.iter().all(|&x| x == 0)
        { 0 }
    else
        { differences.last().unwrap() + extrapolate(&differences) }
}

fn differences(sequence: &[i64]) -> Vec<i64> {
    let mut differences = Vec::new();
    for i in 0..sequence.len() - 1 {
        differences.push(sequence[i + 1] - sequence[i]);
    }
    differences
}

pub fn part_2(input: &str) -> i64 {
    input.lines()
        .map(|line| line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .map(|sequence| sequence.first().unwrap() - extrapolate_backwards(&sequence)).sum()
}

fn extrapolate_backwards(sequence: &[i64]) -> i64 {
    let differences = differences(sequence);
    if differences.iter().all(|&x| x == 0)
        { 0 }
    else
        { differences.first().unwrap() - extrapolate_backwards(&differences) }
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_9.txt");
    assert_eq!(part_1(input), 114)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_9.txt");
    assert_eq!(part_2(input), 2)
}