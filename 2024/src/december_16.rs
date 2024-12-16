
pub fn part_1(input: &str) -> u32 {
    0
}

pub fn part_2(input: &str) -> u32 {
    0
}

#[test]
fn sample_input_part_1_1() {
    let input = include_str!("../input/sample_16_1.txt");
    assert_eq!(part_1(input), 7036)
}

#[test]
fn sample_input_part_1_2() {
    let input = include_str!("../input/sample_16_2.txt");
    assert_eq!(part_1(input), 11048)
}

#[test]
fn sample_input_part_2_1() {
    let input = include_str!("../input/sample_16_1.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn sample_input_part_2_2() {
    let input = include_str!("../input/sample_16_2.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_16.txt");
    assert_eq!(part_1(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_16.txt");
    assert_eq!(part_2(input), 0)
}
