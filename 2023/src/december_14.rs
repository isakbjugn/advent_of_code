pub fn part_1(input: &str) -> u32 {
    columns_to_vecs(input).iter()
        .map(roll_north)
        .map(calculate_load)
        .sum()
}

fn roll_north(column: &Vec<char>) -> Vec<char> {
    let new_column: Vec<Vec<char>> = column.split_inclusive(|c| c == &'#')
        .map(|slice| {
            if slice[0] == '#' {
                return slice.to_vec();
            }
            roll_slice_north(slice.to_vec())
        })
        .collect();
    new_column.into_iter().flatten().collect()
}

fn roll_slice_north(mut slice: Vec<char>) -> Vec<char> {
    slice.sort_by_key(|a| char_order(*a));
    slice
}

// Helper function to determine the order of a character
fn char_order(c: char) -> u32 {
    match c {
        'O' => 1,
        '.' => 2,
        '#' => 3,
        _ => 4, // You can add more cases or a default order for other characters
    }
}

fn calculate_load(column: Vec<char>) -> u32 {
    let length = column.len() as u32;
    column.iter().enumerate()
        .filter_map(|(i, ch)| if *ch == 'O' { Some(length - i as u32) } else { None })
        .sum()
}

fn columns_to_vecs(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();
    let max_length = lines[0].chars().count();
    let mut columns = vec![Vec::new(); max_length];

    for line in lines {
        for (i, ch) in line.chars().enumerate() {
            if i < columns.len() {
                columns[i].push(ch);
            }
        }
    }

    columns
}

pub fn part_2(input: &str) -> i16 {

    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_14.txt");
    assert_eq!(part_1(input), 136)
}

#[test]
fn roll_sample_data_north() {
    let sample = include_str!("../input/sample_14.txt");
    let sample_rolled_north = columns_to_vecs(sample).iter()
        .map(roll_north)
        .collect::<Vec<Vec<char>>>();
    let expected_sample_rolled_north = columns_to_vecs(include_str!("../input/sample_14_rolled_north.txt"));
    assert_eq!(sample_rolled_north, expected_sample_rolled_north)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_14.txt");
    assert_eq!(part_2(input), 0)
}