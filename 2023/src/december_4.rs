pub fn part_1(input: &str) -> i32 {
    input.lines()
        .map(to_two_lists)
        .map(|(winning, owned)| calulate_lottery_points(winning, owned))
        .sum()
}

fn to_two_lists(input: &str) -> (Vec<i8>, Vec<i8>) {
    let numbers = input.split_once(':').unwrap().1;
    //println!("{:?}", numbers.split_once('|').unwrap().0.split_whitespace().collect::<Vec<&str>>());
    let lists: Vec<Vec<i8>> = numbers
        .split('|')
        .map(|number_str| number_str
            .split_whitespace()
            .map(|str| str.parse::<i8>().unwrap())
            .collect())
        .collect();
    (lists.get(0).unwrap().to_vec(), lists.get(1).unwrap().to_vec())
}

fn calulate_lottery_points(winning: Vec<i8>, owned: Vec<i8>) -> i32 {
    match owned
        .into_iter()
        .filter(|&num| winning.contains(&num))
        .count() {
        0 => 0,
        n => 2_i32.pow(n as u32 - 1)
    }
}

pub fn part_2(_input: &str) -> i16 {

    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_4.txt");
    assert_eq!(part_1(input), 13)
}

#[test]
fn sample_input_part_2_1() {
    let input = include_str!("../input/sample_4.txt");
    assert_eq!(part_2(input), 0)
}