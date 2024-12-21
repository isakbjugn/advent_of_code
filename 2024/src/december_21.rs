use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    let codes: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    
    codes.iter().map(|code| {
        let num_pad_moves: Vec<char> = ['A'].iter().chain(code.iter()).tuple_windows()
            .flat_map(|(&digit, &next_digit)| move_on_num_pad(digit, next_digit)).collect();
        let d_pad_moves: Vec<char> = ['A'].iter().chain(num_pad_moves.iter()).tuple_windows()
            .flat_map(|(&digit, &next_digit)| move_on_d_pad(digit, next_digit)).collect();
        let d_pad_moves: Vec<char>= ['A'].iter().chain(d_pad_moves.iter()).tuple_windows()
            .flat_map(|(&digit, &next_digit)| move_on_d_pad(digit, next_digit)).collect();
        println!("Code {:?} solved with {:?} ({}*{})",
                 code.iter().collect::<String>(),
                 d_pad_moves.iter().collect::<String>(),
                 d_pad_moves.len(),
                 code_to_number(code).unwrap()
        );
        d_pad_moves.len() * code_to_number(code).unwrap()
    })
        .sum()
}

fn code_to_number(code: &Vec<char>) -> Option<usize> {
    let numeric_string: String = code
        .into_iter()
        .filter(|c| c.is_ascii_digit())
        .collect();

    // Parse the resulting string into a u64
    numeric_string.parse::<usize>().ok()
}

fn move_on_num_pad(from: char, to: char) -> Vec<char> {
    match (from, to) {
        // rett høyre
        ('7', '8') | ('8', '9') |
        ('4', '5') | ('5', '6') |
        ('1', '2') | ('2', '3') |
                     ('0', 'A') => vec!['>', 'A'],
        
        // rett venstre
        ('8', '7') | ('9', '8') |
        ('5', '4') | ('6', '5') |
        ('2', '1') | ('3', '2') |
                     ('A', '0') => vec!['<', 'A'],
        
        // to høyre
        ('7', '9') |
        ('4', '6') |
        ('1', '3') => vec!['>', '>', 'A'],
        
        // to venstre
        ('9', '7') |
        ('6', '4') |
        ('3', '1') => vec!['<', '<', 'A'],
        
        // rett ned
        ('7', '4') | ('8', '5') | ('9', '6') |
        ('4', '1') | ('5', '2') | ('6', '3') |
        ('2', '0') | ('3', 'A') => vec!['v', 'A'],
        
        // ned høyre (obs! (1, 0) er spesiell!)
        ('7', '5') | ('8', '6') |
        ('4', '2') | ('5', '3') |
                     ('2', 'A' ) => vec!['v', '>', 'A'],
        ('1', '0')               => vec!['>', 'v', 'A'],
        
        // ned venstre
        ('8', '4') | ('9', '5') |
        ('5', '1') | ('6', '2') |
                     ('3', '0') => vec!['<', 'v', 'A'],
        
        // ned, to høyre (obs! (1, A) er unntak!)
        ('7', '6') |
        ('4', '3') |
        ('1', 'A') => vec!['>', '>', 'v', 'A'],
        // ned, to venstre
        ('9', '4') |
        ('6', '1') => vec!['<', '<', 'v', 'A'],
        
        ('7', '1') | ('8', '2') | ('9', '3') |
                     ('5', '0') | ('6', 'A') => vec!['v', 'v', 'A'],
        
        ('7', '2') | ('8', '3') |
        ('4', '0') | ('5', 'A') => vec!['v', 'v', '>', 'A'],
        
        ('8', '1') | ('9', '2') |
        ('6', '0') => vec!['<', 'v', 'v', 'A'],
        ('7', '3') |
        ('4', 'A') => vec!['>', '>', 'v', 'v', 'A'],
        ('9', '1') => vec!['<', '<', 'v', 'v', 'A'],
        ('7', '0') | ('8', 'A') => vec!['>', 'v', 'v', 'v', 'A'],
        
        ('9', '0') => vec!['<', 'v', 'v', 'v', 'A'],
        ('7', 'A') => vec!['>', '>', 'v', 'v', 'v', 'A'],
        
        ('8', '0') | ('9', 'A') => vec!['v', 'v', 'v', 'A'],
        
        ('4', '7') | ('5', '8') | ('6', '9') |
        ('1', '4') | ('2', '5') | ('3', '6') |
                     ('0', '2') | ('A', '3') => vec!['^', 'A'],
        ('4', '8') | ('5', '9') |
        ('1', '5') | ('2', '6') |
                     ('0', '3') => vec!['>', '^', 'A'],
        ('5', '7') | ('6', '8') |
        ('2', '4') | ('3', '5') |
        
        ('0', '1') | ('A', '2') => vec!['^', '<', 'A'],
        
        ('4', '9') |
        ('1', '6') => vec!['>', '>', '^', 'A'],
        ('6', '7') |
        ('3', '4') => vec!['<', '<', '^', 'A'],
        ('A', '1') => vec!['^', '<', '<', 'A'],
        ('1', '7') | ('2', '8') | ('3', '9') |
                     ('0', '5') | ('A', '6') => vec!['^', '^', 'A'],
        ('1', '8') | ('2', '9') |
        ('0', '6') => vec!['^', '^', '>', 'A'],
        ('2', '7') | ('3', '8') |
        
        ('0', '4') => vec!['^', '^', '<', 'A'],
        ('A', '5') => vec!['<', '^', '^', 'A'],
        
        ('1', '9') => vec!['>', '>', '^', '^', 'A'],
        ('3', '7') => vec!['<', '<', '^', '^', 'A'],
        ('A', '4') => vec!['^', '^', '<', '<', 'A'],
        ('0', '8') | ('A', '9') => vec!['^', '^', '^', 'A'],
        ('0', '9') => vec!['^', '^', '^', 'A'],
        ('0', '7') => vec!['^', '^', '^', '<', 'A'],
        ('A', '8') => vec!['<', '^', '^', '^', 'A'],
        ('A', '7') => vec!['^', '^', '^', '<', '<', 'A'],
        
        _ => {
            println!("Tried to go from {} to {}", from, to);
            unreachable!("How did it come to this?")
        }
    }
}

fn move_on_d_pad(from: char, to: char) -> Vec<char> {
    match (from, to) {
        (x, y) if x == y => vec!['A'],
        
        ('A', '<') => vec!['v', '<', '<', 'A'],
        ('A', '^') => vec!['<', 'A'],
        ('A', 'v') => vec!['<', 'v', 'A'],
        ('A', '>') => vec!['v', 'A'],
        
        ('<', '^') => vec!['>', '^', 'A'],
        ('<', 'v') => vec!['>', 'A'],
        ('<', '>') => vec!['>', '>', 'A'],
        ('<', 'A') => vec!['>', '>', '^', 'A'],

        ('v', '<') => vec!['<', 'A'],
        ('v', '^') => vec!['^', 'A'],
        ('v', '>') => vec!['>', 'A'],
        ('v', 'A') => vec!['>', '^', 'A'],
        
        ('^', '<') => vec!['v', '<', 'A'],
        ('^', 'v') => vec!['v', 'A'],
        ('^', '>') => vec!['v', '>', 'A'],
        ('^', 'A') => vec!['>', 'A'],
        
        ('>', '<') => vec!['<', '<', 'A'],
        ('>', 'v') => vec!['<', 'A'],
        ('>', '^') => vec!['<', '^', 'A'],
        ('>', 'A') => vec!['^', 'A'],
        
        _ => {
            println!("On d-pad, tried to go from {} to {}", from, to);
            unreachable!("What about this?")
        }      
    }
}

pub fn part_2(input: &str) -> usize {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_21.txt");
    assert_eq!(part_1(input), 126384)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_21.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_21.txt");
    let result = part_1(input);
    println!("Result: {}", result);
    assert_eq!(result < 276540, true);
    assert_eq!(result < 273976, true);
    assert_eq!(result, 0)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_21.txt");
    assert_eq!(part_2(input), 0)
}
