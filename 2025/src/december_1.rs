pub fn part_1(input: &str) -> u32 {
    let mut dial = 50;
    let mut zeros = 0;
    for line in input.lines() {
        let (direction, value) = line.split_at(1);
        let distance: i32 = value.parse().unwrap();
        dial = match direction {
            "L" => (dial - distance) % 100,
            "R" => (dial + distance) % 100,
            &_ => unreachable!(),
        };
        if dial < 0 {
            dial += 100;
        }
        if dial == 0 { zeros += 1 }
    }
    zeros
}

pub fn part_2(input: &str) -> i32 {
    let mut dial = 50;
    let mut zeros = 0;
    for line in input.lines() {
        let (direction, value) = line.split_at(1);
        let distance: i32 = value.parse().unwrap();
        dial = match direction {
            "L" => {
                let new_value = dial - distance;
                if new_value == 0 {
                    zeros += 1;
                }
                else if new_value < 0 && dial == 0 {
                    let clicks = new_value / 100;
                    zeros -= clicks;
                }
                else if new_value < 0 {
                    let clicks = -1 + new_value / 100;
                    zeros -= clicks;
                }
                new_value % 100
            },
            "R" => {
                let new_value = dial + distance;
                let clicks = new_value / 100;
                zeros += clicks;
                new_value % 100
            },
            &_ => unreachable!(),
        };
        if dial < 0 {
            dial += 100;
        }
    }
    zeros
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_1.txt");
    assert_eq!(part_1(input), 3)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_1.txt");
    assert_eq!(part_1(input), 1052)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_1.txt");
    assert_eq!(part_2(input), 6)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_1.txt");
    assert_eq!(part_2(input), 6295)
}
