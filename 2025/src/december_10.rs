pub fn part_1(input: &str) -> u64 {
    0
}
pub fn part_2(input: &str) -> u64 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_9.txt");
    assert_eq!(part_1(input), 50)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_9.txt");
    assert_eq!(part_1(input), 4781377701)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_9.txt");
    assert_eq!(part_2(input), 24)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_9.txt");
    assert_eq!(part_2(input), 1470616992)
}
