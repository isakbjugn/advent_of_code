
pub fn part_1(input: &str) -> String {
    let mut splitted_input = input.split("\n\n");
    let stack_input = splitted_input.next().unwrap();
    let procedure_input = splitted_input.next().unwrap();

    let mut stacks = create_stacks(stack_input);
    for procedure in procedure_input.lines() {
        let moves = get_moves(procedure);

        for _ in 0..moves.0 {
            let krate = stacks[moves.1].pop().unwrap();
            stacks[moves.2].push(krate);
        }
    }
    stacks.iter().map(|s| *s.last().unwrap()).collect::<Vec<char>>().iter().collect()
}

fn get_moves(procedure: &str) -> (usize, usize, usize) {
    let procedure_parts: Vec<&str>  = procedure.split(' ').collect();
    let moves = procedure_parts[1].parse::<usize>().unwrap();
    let origin_stack = procedure_parts[3].parse::<usize>().unwrap() - 1;
    let end_stack = procedure_parts[5].parse::<usize>().unwrap() - 1;
    (moves, origin_stack, end_stack)
}

fn create_stacks(stack_input: &str) -> Vec<Vec<char>> {
    let stack_count = (stack_input.lines().next().unwrap().chars().count() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..stack_count {
        let stack = Vec::new();
        stacks.push(stack);
    }
    for line in stack_input.lines().rev().skip(1) {
        for idx in 0..stack_count {
            match line.chars().nth(idx*4+1).unwrap() {
                ' ' => continue,
                letter => stacks[idx].push(letter)
            }
        }
    }
    stacks
}

pub fn part_2(input: &str) -> String {
    let mut splitted_input = input.split("\n\n");
    let stack_input = splitted_input.next().unwrap();
    let procedure_input = splitted_input.next().unwrap();

    let mut stacks = create_stacks(stack_input);
    for procedure in procedure_input.lines() {
        let moves = get_moves(procedure);
        let mut krates: Vec<char> = Vec::new();

        for _ in 0..moves.0 {
            krates.push(stacks[moves.1].pop().unwrap());
        }
        for krate in krates.iter().rev() {
            stacks[moves.2].push(*krate);
        }
    }
    stacks.clone().iter().map(|s| *s.last().unwrap()).collect::<Vec<char>>().iter().collect()
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_5.txt");
    assert_eq!(part_1(input), "CMZ")
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_5.txt");
    assert_eq!(part_2(input), "MCD")
}
