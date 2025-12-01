pub fn part_1(input: &str) -> u32 {
    let mut dial = 50;
    let mut zeros = 0;
    for line in input.lines() {
        let (direction, value) = line.split_at(1);
        let value: i32 = value.parse().unwrap();
        dial = match direction {
            "L" => (dial - value) % 100,
            "R" => (dial + value) % 100,
            &_ => unreachable!(),
        };
        if dial < 0 {
            dial += 100;
        }
        if dial == 0 { zeros += 1 }
    }
    zeros
}

pub fn part_2(input: &str) -> u32 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_1.txt");
    assert_eq!(part_1(input), 3)
}