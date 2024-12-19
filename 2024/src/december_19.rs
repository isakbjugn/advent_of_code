use crate::sliceable::Sliceable;

pub fn part_1(input: &str) -> usize {
    let (towels_str, designs_str) = input.split_once("\n\n").expect("Input should contain both towels and designs");
    let towels: Vec<&str> = towels_str.split(", ").collect();
    let designs: Vec<&str> = designs_str.lines().collect();

    designs.into_iter().filter(|design| can_construct_tab(design, &towels)).count()
}

fn can_construct_tab(target: &str, word_bank: &Vec<&str>) -> bool {
    let mut table = vec![false; target.len() + 1];
    table[0] = true;

    for i in 0..=target.len() {
        for &word in word_bank {
            if table[i] && target.contains_at(word, i) {
                table[i + word.len()] = true;
            }
        }
    }

    table[target.len()]
}

pub fn part_2(input: &str) -> usize {
    let (towels_str, designs_str) = input.split_once("\n\n").expect("Input should contain both towels and designs");
    let towels: Vec<&str> = towels_str.split(", ").collect();
    let designs: Vec<&str> = designs_str.lines().collect();

    designs.into_iter().map(|design| count_construct_tab(design, &towels) as usize).sum()
}

fn count_construct_tab(target: &str, word_bank: &Vec<&str>) -> u64 {
    let mut table = vec![0; target.len() + 1];
    table[0] = 1;

    for i in 0..=target.len() {
        for &word in word_bank {
            if table[i] > 0 && target.contains_at(word, i) {
                table[i + word.len()] += table[i];
            }
        }
    }

    table[target.len()]
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_19.txt");
    assert_eq!(part_1(input), 6)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_19.txt");
    assert_eq!(part_2(input), 16)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_19.txt");
    assert_eq!(part_1(input), 209)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_19.txt");
    assert_eq!(part_2(input), 777669668613191)
}
