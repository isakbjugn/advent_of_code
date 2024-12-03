use regex::Regex;

pub fn part_1(input: &str) -> u32 {
    let pattern = r"mul\((\d+),(\d+)\)";

    // Compile the regular expression
    let re = Regex::new(pattern).unwrap();
    let mut sum = 0;

    // Find all matches
    for caps in re.captures_iter(input) {
        let num1 = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let num2 = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        sum += num1 * num2;
    }
    sum
}

pub fn part_2(input: &str) -> usize {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_3.txt");
    assert_eq!(part_1(input), 161)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_3.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_3.txt");
    assert_eq!(part_1(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_3.txt");
    assert_eq!(part_2(input), 0)
}
