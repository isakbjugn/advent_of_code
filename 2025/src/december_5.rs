pub fn part_1(input: &str) -> u64 {
    let (ranges_str, ids_str) = input.split_once("\n\n").unwrap();
    let ranges: Vec<(u64, u64)> = ranges_str
        .lines()
        .map(|line| {
            let (start, inclusive_end) = line.split_once("-").unwrap();
            (start.parse().unwrap(), inclusive_end.parse().unwrap())
        })
        .collect();
    let ids: Vec<u64> = ids_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    ids.into_iter().filter(|id| {
        ranges.iter().any(|(start, end)| id >= start && id <= end)
    }).count() as u64
}

pub fn part_2(input: &str) -> u64 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_5.txt");
    assert_eq!(part_1(input), 3)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_5.txt");
    assert_eq!(part_1(input), 623)
}

#[test]
fn sample_input_part_2() {
    //let input = include_str!("../input/sample_5.txt");
    //assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_2() {
    //let input = include_str!("../input/input_5.txt");
    //assert_eq!(part_2(input), 0)
}
