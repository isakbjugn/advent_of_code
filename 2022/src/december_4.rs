use std::str::Split;

pub fn part_1(input: &str) -> i32 {
    let mut redundant_pairs = 0;
    let mut pair: Split<&str>;
    for line in input.lines() {
        pair = line.split(",");
        if containing_pair(pair.next().unwrap(), pair.next().unwrap()) {
            redundant_pairs += 1;
        }
    }
    redundant_pairs
}

fn pair_to_tuple(s: &str) -> (i32, i32) {
    let mut pair = s.split("-");
    let start = pair.next().unwrap().parse::<i32>().unwrap();
    let end = pair.next().unwrap().parse::<i32>().unwrap();
    return (start, end)
}

fn containing_pair(s1: &str, s2: &str) -> bool {
    let first = pair_to_tuple(s1);
    let second = pair_to_tuple(s2);
    if first.0 >= second.0 && first.1 <= second.1 {
        return true
    }
    if second.0 >= first.0 && second.1 <= first.1 {
        return true
    }
    false
}

fn overlapping_pair(s1: &str, s2: &str) -> bool {
    let first = pair_to_tuple(s1);
    let second = pair_to_tuple(s2);
    if first.0 <= second.0 && first.1 >= second.0 {
        return true
    }
    if first.0 <= second.1 && first.1 >= second.1 {
        return true
    }
    if second.0 <= first.0 && second.1 >= first.0 {
        return true
    }
    if second.0 <= first.1 && second.1 >= first.1 {
        return true
    }
    false
}

pub fn part_2(input: &str) -> i32 {
    let mut redundant_pairs = 0;
    let mut pair: Split<&str>;
    for line in input.lines() {
        pair = line.split(",");
        if overlapping_pair(pair.next().unwrap(), pair.next().unwrap()) {
            redundant_pairs += 1;
        }
    }
    redundant_pairs
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_4.txt");
    assert_eq!(part_1(input), 2)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_4.txt");
    assert_eq!(part_2(input), 4)
}