use std::collections::HashSet;

pub fn part_1(input: &str) -> i32 {
    let mut kars: Vec<char> = Vec::new();
    for (i, kar) in input.chars().enumerate() {
        if kars.len() > 3 {
            kars.remove(0);
        }
        kars.push(kar);
        let kar_set: HashSet<char> = HashSet::from_iter(kars.iter().cloned());
        if kar_set.len() == 4 { return i as i32 + 1 }
    }
    0
}

pub fn part_2(input: &str) -> i32 {
    let mut kars: Vec<char> = Vec::new();
    for (i, kar) in input.chars().enumerate() {
        if kars.len() > 13 {
            kars.remove(0);
        }
        kars.push(kar);
        let kar_set: HashSet<char> = HashSet::from_iter(kars.iter().cloned());
        if kar_set.len() == 14 { return i as i32 + 1 }
    }
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_6.txt");
    assert_eq!(part_1(input), 7)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_6.txt");
    assert_eq!(part_2(input), 19)
}
