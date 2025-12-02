pub fn part_1(input: &str) -> u64 {
    let ranges = input.split(',');
    let mut sum = 0;
    for range in ranges {
        let (first, last) = range.split_once('-').expect("Burde være bindestrek som skilletegn");
        let first = first.parse::<u64>().expect("Må være heltall");
        let last = last.parse::<u64>().expect("Må være heltall");
        for number in first..=last {
            if is_repeated(number) {
                sum += number;
            }
        }
    }
    sum
}

fn is_repeated(number: u64) -> bool {
    match number.to_string().len() % 2 {
        0 => {
            let s = number.to_string();
            let mid = s.len() / 2;
            let first_half = &s[..mid];
            let second_half = &s[mid..];
            first_half == second_half
        }
        _ => false,
    }
}

pub fn part_2(input: &str) -> u32 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part_1(input), 1227775554)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_2.txt");
    assert_eq!(part_1(input), 64215794229)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_2.txt");
    //assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_2.txt");
    //assert_eq!(part_2(input), 0)
}
