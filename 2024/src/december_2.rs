pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(into_increasing)
        .filter(is_report_safe)
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(into_increasing)
        .filter(|report| {
            for (drop_index, _) in report.iter().enumerate() {
                let mut shorter_report = report.clone();
                shorter_report.remove(drop_index);
                match is_report_safe(&shorter_report) {
                    true => return true,
                    false => continue,
                }
            }
            false
        })
        .count()
}

fn into_increasing(report: Vec<u32>) -> Vec<u32> {
    match report.first().unwrap() < report.last().unwrap() {
        true => report,
        false => report.into_iter().rev().collect(),
    }
}

fn is_report_safe(report: &Vec<u32>) -> bool {
    for (index, &level) in report.iter().enumerate() {
        if let Some(&next) = report.get(index + 1) {
            match next as i32 - level as i32 {
                difference if difference < 1 => return false,
                difference if difference > 3 => return false,
                _ => continue,
            }
        }
    }
    true
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part_1(input), 2)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part_2(input), 4)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_2.txt");
    assert_eq!(part_1(input), 639)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_2.txt");
    assert_eq!(part_2(input), 674)
}
