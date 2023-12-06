use std::iter::zip;

pub fn part_1(input: &str) -> i64 {
    let mut lines = input.lines();
    let times: Vec<i64> = lines.next().unwrap().split_whitespace().skip(1)
        .map(|time| time.parse::<i64>().unwrap()).collect();
    let records: Vec<i64> = lines.next().unwrap().split_whitespace().skip(1)
        .map(|time| time.parse::<i64>().unwrap()).collect();
    let races: Vec<(i64, i64)> = zip(times, records).collect();

    races.iter().map(|(time, distance)| count_winning_ways(*time, *distance)).product()
}

fn count_winning_ways(time: i64, record: i64) -> i64 {
    (1..time)
        .map(|button_pressed_time| button_pressed_time * (time - button_pressed_time))
        .filter(|distance_covered| *distance_covered > record)
        .count() as i64
}

pub fn part_2(input: &str) -> i64 {
    let mut lines = input.lines();
    let time_str = lines.next().unwrap().split_once(':').unwrap().1.replace(' ', "");
    let time = time_str.parse::<i64>().unwrap_or_else(|_| panic!("{} is not a number", time_str));
    let record_str = lines.next().unwrap().split_once(':').unwrap().1.replace(' ', "");
    let record = record_str.parse::<i64>().unwrap_or_else(|_| panic!("{} is not a number", record_str));
    count_winning_ways(time, record)
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_6.txt");
    assert_eq!(part_1(input), 288)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_6.txt");
    assert_eq!(part_2(input), 71503)
}