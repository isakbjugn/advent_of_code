
pub fn part_1(input: &str) -> i32 {
    let lines = input.lines();
    let mut score = 0;
    for round in lines {
        score += eval_round(round.chars().nth(0).unwrap(), round.chars().nth(2).unwrap());
    }
    return score
}

fn eval_round(opponent: char, me: char) -> i32 {
    let outcome = match (opponent, me) {
        ('A', 'X') => 3,
        ('A', 'Y') => 6,
        ('A', 'Z') => 0,
        ('B', 'X') => 0,
        ('B', 'Y') => 3,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        ('C', 'Y') => 0,
        ('C', 'Z') => 3,
    _ => panic!(),
};
    let shape = match me {
    'X' => 1,
    'Y' => 2,
    'Z' => 3,
    _ => panic!(),
};
    return outcome + shape;
}


pub fn part_2(input: &str) -> i32 {
    let lines = input.lines();
    let mut score = 0;
    for round in lines {
        score += find_move_and_eval_round(round.chars().nth(0).unwrap(), round.chars().nth(2).unwrap());
    }
    return score
}

fn find_move_and_eval_round(opponent: char, outcome: char) -> i32 {
    let my_move = match (opponent, outcome) {
        ('A', 'X') => 'C',
        ('A', 'Y') => 'A',
        ('A', 'Z') => 'B',
        ('B', 'X') => 'A',
        ('B', 'Y') => 'B',
        ('B', 'Z') => 'C',
        ('C', 'X') => 'B',
        ('C', 'Y') => 'C',
        ('C', 'Z') => 'A',
    _ => panic!(),
};
    let move_score = match my_move {
    'A' => 1,
    'B' => 2,
    'C' => 3,
    _ => panic!(),
};
    let outcome_score = match outcome {
    'X' => 0,
    'Y' => 3,
    'Z' => 6,
    _ => panic!(),
};
    return outcome_score + move_score;
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part_1(input), 15)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_2.txt");
    assert_eq!(part_2(input), 12)
}
