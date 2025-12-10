use std::collections::{HashSet, VecDeque};
use good_lp::*;

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

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(to_buttons_and_joltage)
        .map(|(buttons, joltages)| solve_ilp(&buttons, &joltages))
        .map(|solution| {
            solution.iter().sum::<u32>()
        })
        .sum()
}

fn to_buttons_and_joltage(line: &str) -> (Vec<Vec<u32>>, Vec<u32>) {
    let end_square_bracket_index = line.find("]").unwrap();
    let (_, rest) = line.split_at(end_square_bracket_index + 1);
    let start_curly_brace_index = rest.trim().find("{").unwrap();
    let (button_str, joltage_str) = rest.trim().split_at(start_curly_brace_index);

    let buttons: Vec<Vec<u32>> = button_str
        .split_whitespace()
        .map(|str|
            str.chars().filter_map(|c| c.to_digit(10)).collect()
        )
        .collect();
    let joltages: Vec<u32> = joltage_str
        .trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect();
    let mut matrix_a = vec![vec![0; buttons.len()]; joltages.len()];
    for (col, button) in buttons.iter().enumerate() {
        for &row in button {
            matrix_a[row as usize][col] = 1;
        }
    }

    (matrix_a, joltages)
}

fn solve_ilp(matrix_a: &[Vec<u32>], vec_b: &[u32]) -> Vec<u32> {
    let rows = matrix_a.len();
    let cols = matrix_a[0].len();

    let mut vars = variables!();
    let mut x = Vec::new();

    // Create non-negative integer variables in vector x
    for _ in 0..cols {
        x.push(vars.add(variable().integer().min(0)));
    }

    let objective: Expression = x.iter().copied().sum();

    // Build constraints A * x = target
    let mut pb = vars.minimise(objective).using(default_solver);

    for r in 0..rows {
        let expr: Expression = matrix_a[r].iter()
            .enumerate()
            .map(|(c, coef)| *coef as i32 * x[c])
            .sum();

        pb = pb.with(expr.eq(vec_b[r] as f64));
    }

    let solution = pb.solve().unwrap();
    (0..cols).map(|col| solution.value(x[col]) as u32).collect()
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
    assert_eq!(part_2(input), 33)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_10.txt");
    assert_eq!(part_2(input), 16978)
}
