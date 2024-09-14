use itertools::Itertools;
use regex::{Match, Regex};

pub fn part_1(input: &str) -> usize {
    input.lines()
        .map(to_record_and_groups)
        .map(|(record, groups)| count_arrangements(record, groups))
        .sum()
}

fn to_record_and_groups(line: &str) -> (&str, Vec<usize>) {
    let (record, groups) = line.split_once(' ').unwrap();
    (record,groups.split(',').map(|c| c.parse::<usize>().unwrap()).collect::<Vec<usize>>())
}

fn count_arrangements(record: &str, groups: Vec<usize>) -> usize {
    generate_arrangements(record, groups.iter().sum()).iter()
        .filter(|&record| {
            valid_arrangement(record, &groups)
        })
        .count()
}

fn generate_arrangements(record: &str, damaged: usize) -> Vec<String> {
    let unknowns = Regex::new(r"\?").unwrap().find_iter(record)
        .map(|mat: Match| mat.start())
        .collect();
    let knowns = record.matches('#').count();
    combinations(unknowns, damaged - knowns).iter()
        .map(|swap_indeces| {
            let mut chars: Vec<char> = record.chars().collect();
            swap_indeces.iter().for_each(|&index| {
                chars[index] = '#'
            });
            String::from_iter(chars)
        } )
        .collect()
}

fn combinations(pool: Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    pool.iter().copied().combinations(n).collect()
}

fn valid_arrangement(record: &str, groups: &Vec<usize>) -> bool {
    let segments: Vec<usize> = Regex::new(r"#+").unwrap()
        .find_iter(record).map(|mat| mat.as_str().len()).collect();
    segments == *groups
}

pub fn part_2(input: &str) -> i16 {

    0
}

#[test]
fn test_valid_arrangement() {
    let records = "#....######..#####.";
    let groups = [1, 6, 5].to_vec();
    assert!(valid_arrangement(records, &groups))
}

#[test]
fn specific_line_part_1() {
    assert_eq!(part_1("..??#???##??#?? 4,2,2"), 4);
    assert_eq!(part_1(".#?????????.?. 9,1"), 1);
    assert_eq!(part_1(".????#..??#? 4,2"), 2);
    assert_eq!(part_1("??#.#???#? 2,1,1"), 1);
    assert_eq!(part_1("?#??.???#?#????? 4,1,1,2,3"), 2);
    assert_eq!(part_1(".??????.?##. 1,3"), 6);
}

#[test]
fn first_line() {
    let row = "???.### 1,1,3";
    let (record, groups) = to_record_and_groups(row);
    let arrangements = count_arrangements(record, groups);
    assert_eq!(arrangements, 1)
}

#[test]
fn second_line() {
    let row = ".??..??...?##. 1,1,3";
    let (record, groups) = to_record_and_groups(row);
    let arrangements = count_arrangements(record, groups);
    assert_eq!(arrangements, 4)
}

#[test]
fn third_line() {
    let row = "?#?#?#?#?#?#?#? 1,3,1,6";
    let (record, groups) = to_record_and_groups(row);
    let arrangements = count_arrangements(record, groups);
    assert_eq!(arrangements, 1)
}

#[test]
fn fourth_line() {
    let row = "????.#...#... 4,1,1";
    let (record, groups) = to_record_and_groups(row);
    let arrangements = count_arrangements(record, groups);
    assert_eq!(arrangements, 1)
}

#[test]
fn fifth_line() {
    let row = "????.######..#####. 1,6,5";
    let (record, groups) = to_record_and_groups(row);
    let arrangements = count_arrangements(record, groups);
    assert_eq!(arrangements, 4)
}

#[test]
fn sixth_line() {
    let row = "?###???????? 3,2,1";
    let (record, groups) = to_record_and_groups(row);
    let arrangements = count_arrangements(record, groups);
    assert_eq!(arrangements, 10)
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_12.txt");
    assert_eq!(part_1(input), 21)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_12.txt");
    assert_eq!(part_2(input), 0)
}