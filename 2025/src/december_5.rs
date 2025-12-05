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
    let (ranges_str, _) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<(u64, u64)> = ranges_str
        .lines()
        .map(|line| {
            let (start, inclusive_end) = line.split_once("-").unwrap();
            (start.parse().unwrap(), inclusive_end.parse().unwrap())
        })
        .collect();

    let mut changed = true;
    while changed {
        changed = false;
        let mut new_ranges: Vec<(u64, u64)> = Vec::new();
        for range in &ranges {
            let mut overlapping = false;
            for new_range in &mut new_ranges {
                if !(range.1 < new_range.0 || range.0 > new_range.1) {
                    new_range.0 = new_range.0.min(range.0);
                    new_range.1 = new_range.1.max(range.1);
                    overlapping = true;
                    changed = true;
                    break;
                }
            }
            if !overlapping {
                new_ranges.push(*range);
            }
        }
        ranges = new_ranges;
    }

    ranges.into_iter().map(|range| range.1 - range.0 + 1).sum()
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
    let input = include_str!("../input/sample_5.txt");
    assert_eq!(part_2(input), 14)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_5.txt");
    assert_eq!(part_2(input), 353507173555373)
}
