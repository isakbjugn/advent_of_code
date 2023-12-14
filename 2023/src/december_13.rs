use itertools::Itertools;

pub fn part_1(input: &str) -> u16 {
    input.split("\n\n")
        .map(calculate_reflection)
        .sum()
}

fn calculate_reflection(pattern: &str) -> u16 {
    //println!("original:");
    if let Some(n) = find_mirror_axis(pattern) { return 100 * n as u16 }

    let pattern = rotate_string_90_degrees_clockwise(pattern);
    //println!("rotated 1/4:");
    if let Some(n) = find_mirror_axis(&pattern) { return n as u16 }

    let pattern = rotate_string_90_degrees_clockwise(&pattern);
    //println!("upside-down:");
    if let Some(n) = find_mirror_axis(&pattern) { return 100 * (pattern.lines().count() -  n) as u16 }

    let pattern = rotate_string_90_degrees_clockwise(&pattern);
    //println!("rotated 3/4:");
    if let Some(n) = find_mirror_axis(&pattern) { return (pattern.lines().count() - n) as u16 }

    0
}

fn find_mirror_axis(pattern: &str) -> Option<usize> {
    let length = pattern.lines().count();

    for n in 0..length {
        let (motif, rest) = split_at_index(pattern, n);
        //println!("rest: {:?}", rest);
        let flipped = flip(&motif);

        let merged = match rest {
            Some(rest) => [motif, flipped, rest].join("\n"),
            None => [motif, flipped.lines().take(length - n).join("\n")].join("\n")
        };

        //println!("{}\n", merged);
        if merged == pattern {
            return Some(n);
        }
    }

    None
}

fn split_at_index(str: &str, index: usize) -> (String, Option<String>) {
    let first = str.lines().take(index).join("\n");
    if index * 2 >= str.lines().count() {
        return (first, None);
    }
    let rest = str.lines().skip(index * 2).join("\n");
    (first, Some(rest))
}

fn flip(str: &str) -> String {
    str.lines().rev().join("\n")
}

fn rotate_string_90_degrees_clockwise(input: &str) -> String {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let mut rotated = Vec::new();

    for i in 0..lines[0].len() {
        let mut new_line = String::new();
        for line in lines.iter().rev() {
            if let Some(c) = line.chars().nth(i) {
                new_line.push(c);
            }
        }
        rotated.push(new_line);
    }

    rotated.join("\n")
}

fn rotate_string_90_degrees_counter_clockwise(input: &str) -> String {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let num_columns = lines[0].len();
    let mut rotated = Vec::new();

    for i in (0..num_columns).rev() {
        let mut new_line = String::new();
        for line in &lines {
            if let Some(c) = line.chars().nth(i) {
                new_line.push(c);
            }
        }
        rotated.push(new_line);
    }

    rotated.join("\n")
}

pub fn part_2(_input: &str) -> i16 {

    0
}

#[test]
fn test_rotated_string_counter_clockwise() {
    let multiline_string = "1234";
    let rotated = rotate_string_90_degrees_counter_clockwise(multiline_string);
    let expected = "4\n3\n2\n1";
    assert_eq!(rotated, expected)
}

#[test]
fn test_rotated_twoline_counter_string_clockwise() {
    let multiline_string = "1234\n5678";
    let rotated = rotate_string_90_degrees_counter_clockwise(multiline_string);
    let expected = "48\n37\n26\n15";
    assert_eq!(rotated, expected)
}

#[test]
fn test_rotated_string_clockwise() {
    let multiline_string = "1234";
    let rotated = rotate_string_90_degrees_clockwise(multiline_string);
    let expected = "1\n2\n3\n4";
    assert_eq!(rotated, expected)
}

#[test]
fn test_rotated_twoline_string_clockwise() {
    let multiline_string = "1234\n5678";
    let rotated = rotate_string_90_degrees_clockwise(multiline_string);
    let expected = "51\n62\n73\n84";
    assert_eq!(rotated, expected)
}

#[test]
fn test_rotated_multiline_string_clockwise() {
    let multiline_string =
        "#.##..##.\n\
         ..#.##.#.\n\
         ##......#\n\
         ##......#\n\
         ..#.##.#.\n\
         ..##..##.\n\
         #.#.##.#.";
    let rotated = rotate_string_90_degrees_clockwise(multiline_string);
    let expected =
        "#..##.#\n\
         ...##..\n\
         ###..##\n\
         .#....#\n\
         #.#..#.\n\
         #.#..#.\n\
         .#....#\n\
         ###..##\n\
         ...##..";

    assert_eq!(rotated, expected)
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_13.txt");
    assert_eq!(part_1(input), 405)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_13.txt");
    assert_eq!(part_2(input), 0)
}