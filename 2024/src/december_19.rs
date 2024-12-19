
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

pub trait Sliceable {
    fn substr(&self, start_idx: usize, length: usize) -> Option<&str>;
    fn contains_at(&self, substr: &str, idx: usize) -> bool;
}

impl Sliceable for &str {
    fn substr(&self, start_idx: usize, length: usize) -> Option<&str> {
        let end_idx = start_idx + length;
        if self.len() < end_idx { return None }
        Some(&self[start_idx..end_idx])
    }

    fn contains_at(&self, substr: &str, idx: usize) -> bool {
        matches!(self.substr(idx, substr.len()), Some(str) if str == substr)
    }
}

pub fn part_2(input: &str) -> u32 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_19.txt");
    assert_eq!(part_1(input), 6)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_19.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_19.txt");
    assert_eq!(part_1(input), 209)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_19.txt");
    assert_eq!(part_2(input), 0)
}
