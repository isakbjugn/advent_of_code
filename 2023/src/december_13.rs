use colored::Colorize;
use itertools::Itertools;
use crate::december_13::Orientation::{Normal, Rotated};

pub fn part_1(input: &str) -> u16 {
    input.split("\n\n")
        .filter_map(|pattern| calculate_reflection(pattern, None))
        .map(|reflection| reflection.value())
        .sum()
}

pub fn part_2(input: &str) -> u16 {
    input.split("\n\n")
        .map(clean_and_calculate_reflection)
        .map(|reflection| reflection.value())
        .sum()
}

fn clean_and_calculate_reflection(pattern: &str) -> Reflection {
    let original_reflection = calculate_reflection(pattern, None)
        .expect("Should always be an initial reflection");

    println!("Original reflection {:?}", original_reflection);

    for (n, c) in pattern.chars().enumerate() {
        let clean_pattern = match c {
            '#' => switch_nth_char(pattern, n, '.'),
            '.' => switch_nth_char(pattern, n, '#'),
            _ => continue
        };

        //println!("Clean pattern:\n{}", clean_pattern);

        match calculate_reflection(&clean_pattern, Some(&original_reflection)) {
            Some(reflection) if reflection != original_reflection => {
                //println!("Old reflection: {:?}", original_reflection);
                //println!("New reflection: {:?}", reflection);
                //let correct_index = to_index_after_rotation(pattern, n, reflection.orientation);
                //print_with_color(&clean_pattern, correct_index);
                return reflection
            }
            _ => continue
        }
    }
    unreachable!("No smudge found")
}

fn print_with_color(pattern: &str, n: usize) {
    println!("Smudge at index {}", n);
    let first = pattern.chars().take(n).collect::<String>();
    let char = pattern.chars().nth(n).unwrap();
    let rest = pattern.chars().skip(n + 1).collect::<String>();
    println!("{}{}{}", first, char.to_string().red(), rest);
}

fn to_index_after_rotation(pattern: &str, n: usize, orientation: Orientation) -> usize {
    let (width, height) =
        (pattern.split_once('\n').unwrap().0.len() + 1, pattern.lines().count());

    println!("width: {}, height: {}", width, height);

    let coordinates = (n % width, n / width);
    println!("coordinates: {:?}", coordinates);
    let rotated_coordinates = rotate_index(coordinates, height, orientation);
    println!("rotated coordinates: {:?}", rotated_coordinates);
    let index = (rotated_coordinates.1 * width) + rotated_coordinates.0;
    println!("index: {}", index);
    index
}

fn rotate_index(coordinate: (usize, usize), height: usize, orientation: Orientation) -> (usize, usize) {
    // rotate when coordinate is (x, y), where x is horizontal and y is vertical
    match orientation {
        Normal => coordinate,
        Rotated => (height - 1 - coordinate.1, coordinate.0),
    }
}

#[test]
fn rotates_coordinates_origin_square() {
    // coordinates is tuple (x, y), where x is horizontal and y is vertical
    assert_eq!(rotate_index((0, 0), 3, Normal), (0, 0));
    assert_eq!(rotate_index((0, 0), 3, Rotated), (2, 0));
}

#[test]
fn rotates_coordinates_not_origin_square() {
  // Test where width and height are at least 4, and where x != y, and neither is 0
    assert_eq!(rotate_index((1, 2), 5, Normal), (1, 2));
    assert_eq!(rotate_index((1, 2), 5, Rotated), (2, 1));
}

#[test]
fn rotates_coordinates_not_origin_rectangle() {
    // Test with different width and height, at least 4, and where x != y, and neither is 0
    assert_eq!(rotate_index((1, 2),  7, Normal), (1, 2));
    assert_eq!(rotate_index((1, 2),  7, Rotated), (4, 1));
}

#[test]
fn correctly_sets_index_after_rotation_with_example_pattern() {
    let pattern =
        "#.##..##.\n\
         ..#.##.#.\n\
         ##......#\n\
         ##......#\n\
         ..#.##.#.\n\
         ..##..##.\n\
         #.#.##.#.";

    assert_eq!(to_index_after_rotation(pattern, 0, Normal), 0);
    assert_eq!(to_index_after_rotation(pattern, 3, Rotated), 36);
}

fn switch_nth_char(pattern: &str, n: usize, character: char) -> String {
    let mut chars: Vec<char> = pattern.chars().collect();
    chars[n] = character;
    chars.iter().collect()
}

fn calculate_reflection(pattern: &str, ignore: Option<&Reflection>) -> Option<Reflection> {
    let pattern = pattern.trim();
    if let Some(n) = find_mirror_axis(pattern, Normal, ignore) {
        return Some(Reflection::new(n, Normal))
    }

    let pattern = rotate_string_90_degrees_clockwise(pattern);
    if let Some(n) = find_mirror_axis(&pattern, Rotated, ignore) {
        return Some(Reflection::new(n, Rotated))
    }

    None
}

#[derive(Debug, PartialEq, Eq)]
struct Reflection {
    index: usize,
    orientation: Orientation
}

impl Reflection {
    pub fn new(index: usize, orientation: Orientation) -> Self {
        Self { index, orientation }
    }
    fn value(&self) -> u16 {
        match self.orientation {
            Normal => 100 * self.index as u16,
            Rotated => self.index as u16,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Orientation {
    Normal,
    Rotated,
}

fn find_mirror_axis(pattern: &str, orientation: Orientation, ignore: Option<&Reflection>) -> Option<usize> {
    let length = pattern.lines().count();

    for n in 1..length {
        let reflection = Reflection::new(n, orientation);
        if let Some(reflection_to_ignore) = ignore {
            if reflection_to_ignore == &reflection { continue }
        }

        let (motif, rest) = split_at_index(pattern, n);
        let flipped = flip(&motif);

        //println!("pattern for {:?} when n={}:\n{}", orientation, n, pattern);
        //println!("Parts of mirror pattern:");
        //println!("{}", motif);
        let merged = match rest {
            Some(positive_rest) => {
                //println!("{}", flipped.red());
                //println!("{}", positive_rest.blue());
                [motif.clone(), flipped.clone(), positive_rest].join("\n")
            },
            None => {
                //println!("{}", flipped.lines().take(length - n).join("\n").red());
                [motif.clone(), flipped.clone().lines().take(length - n).join("\n")].join("\n")
            }
        };

        if merged.trim() == pattern.trim() {
            //println!("Speilbildene er like");
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
fn sample_input_part_1_1_1() {
    let input = include_str!("../input/sample_13_1_1.txt");
    assert_eq!(part_1(input), 5)
}

#[test]
fn sample_input_part_1_1_2() {
    let input = include_str!("../input/sample_13_1_2.txt");
    assert_eq!(part_1(input), 400)
}

#[test]
fn sample_input_part_1_2() {
    let input = include_str!("../input/sample_13_2.txt");
    assert_eq!(part_1(input), 200)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_13.txt");
    assert_eq!(part_2(input), 400)
}

#[test]
fn sample_input_part_2_2() {
    let input = include_str!("../input/sample_13_2.txt");
    assert_eq!(part_2(input), 9)
}

#[test]
fn sample_input_part_2_3() {
    let input = include_str!("../input/sample_13_3.txt");
    assert_eq!(part_1(input), 2)
}

#[test]
fn sample_input_part_2_4() {
    let input = include_str!("../input/sample_13_3.txt");
    assert_eq!(part_2(input), 1100)
}