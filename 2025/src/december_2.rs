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
    let string = number.to_string();
    let (first_half, second_half) = string.as_str().split_at(string.len() / 2);
    first_half == second_half
}

pub fn part_2(input: &str) -> u64 {
    let ranges = input.split(',');
    let mut sum = 0;
    for range in ranges {
        let (first, last) = range.split_once('-').expect("Burde være bindestrek som skilletegn");
        let first = first.parse::<u64>().expect("Må være heltall");
        let last = last.parse::<u64>().expect("Må være heltall");
        for number in first..=last {
            if is_repeated_any_number_of_times(number) {
                sum += number;
            }
        }
    }
    sum
}

fn is_repeated_any_number_of_times(number: u64) -> bool {
    let string = number.to_string();
    for repeated_length in 1..=string.len()/2 {
        let mut chunks = string.as_bytes().chunks(repeated_length);
        if let Some(first) = chunks.next() {
            if chunks.all(|chunk| chunk == first) {
                return true;
            }
        }
    }
    false
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
    assert_eq!(part_2(input), 4174379265)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_2.txt");
    assert_eq!(part_2(input), 85513235135)
}
