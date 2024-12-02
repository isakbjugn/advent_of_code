use itertools::{Itertools, PeekingNext};

pub fn part_1(input: &str) -> usize {
   input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
        )
        .filter(|report| {
            println!("{:?}", report);
            let length = report.iter().len();
            let increasing = report.first().unwrap() < report.last().unwrap();
            
            match increasing {
                true => {
                    for (index, &level) in report.iter().enumerate() {
                        if let Some(&next) = report.get(index + 1) {
                            println!("Comparing {} and {} as increasing", level, next);
                            match next as i32 - level as i32 {
                                difference if difference < 1 => { return false },
                                difference if difference > 3 => { return false },
                                _ => continue,
                            }
                        }
                    }
                    true
                },
                false => {
                    for (index, &level) in report.iter().rev().enumerate() {
                        if let Some(&next) = report.get(length - index) {
                            println!("Comparing {} and {} as decreasing", level, next);
                            match level as i32 - next as i32 {
                                difference if difference < 1 => { return false },
                                difference if difference > 3 => { return false },
                                _ => continue,
                            }
                        }
                    }
                    true
                }
            }
            
        })
        .count()
}

pub fn part_2(input: &str) -> u32 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part_1(input), 2)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_1.txt");
    assert_eq!(part_1(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_1.txt");
    assert_eq!(part_2(input), 0)
}
