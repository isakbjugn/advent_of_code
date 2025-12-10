use std::collections::{HashSet, VecDeque};

pub fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(to_indicators_and_buttons)
        .map(fewest_presses)
        .sum()
}

fn to_indicators_and_buttons(line: &str) -> (Vec<char>, Vec<Vec<u32>>) {
    let end_square_bracket_index = line.find("]").unwrap();
    let (indicator_str, rest) = line.split_at(end_square_bracket_index + 1);
    let start_curly_brace_index = rest.trim().find("{").unwrap();
    let (button_str, _) = rest.trim().split_at(start_curly_brace_index);

    let indicators: Vec<char> = indicator_str.chars().filter(|&c| c == '.' || c == '#').collect();
    let buttons: Vec<Vec<u32>> = button_str.split_whitespace().map(|str|
        str.chars().filter_map(|c| c.to_digit(10)).collect()
    ).collect();

    (indicators, buttons)
}

fn fewest_presses((indicators, buttons): (Vec<char>, Vec<Vec<u32>>)) -> u64 {
    let num_lights = indicators.len();
    let start = vec!['.'; num_lights];

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start.clone(), 0));
    visited.insert(start);

    while let Some((state, dist)) = queue.pop_front() {
        if state == *indicators {
            return dist;
        }

        for button in &buttons {
            let new_state = press_button(&state, &button);
            if visited.insert(new_state.clone()) {
                queue.push_back((new_state, dist + 1));
            }
        }
    }

    0
}

fn press_button(lights: &Vec<char>, button: &Vec<u32>) -> Vec<char> {
    let mut new_lights = lights.clone();
    for &index in button {
        let idx = index as usize;
        if idx < new_lights.len() {
            new_lights[idx] = if new_lights[idx] == '.' { '#' } else { '.' };
        }
    }
    new_lights
}

pub fn part_2(input: &str) -> u64 {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_10.txt");
    assert_eq!(part_1(input), 7)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_10.txt");
    assert_eq!(part_1(input), 455)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_10.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_10.txt");
    assert_eq!(part_2(input), 0)
}
