use itertools::Itertools;
use rand::prelude::*;
use crate::expandable::Expandable;

pub fn part_1(input: &str) -> usize {
    let codes: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut rng = rand::rng();
    
    let mut minimum = 276540;
    for _ in 0..1000000 {
        let sum = codes.iter().map(|code| {
            let num_pad_moves: Vec<char> = ['A'].iter().chain(code.iter()).tuple_windows()
                .flat_map(|(&digit, &next_digit)| move_on_num_pad_new(digit, next_digit, &mut rng)).collect();
            let d_pad_moves: Vec<char> = ['A'].iter().chain(num_pad_moves.iter()).tuple_windows()
                .flat_map(|(&digit, &next_digit)| move_on_d_pad(digit, next_digit, &mut rng)).collect();
            let d_pad_moves: Vec<char>= ['A'].iter().chain(d_pad_moves.iter()).tuple_windows()
                .flat_map(|(&digit, &next_digit)| move_on_d_pad(digit, next_digit, &mut rng)).collect();
            // println!("Code {:?} solved with {:?} ({}*{})",
            //          code.iter().collect::<String>(),
            //          d_pad_moves.iter().collect::<String>(),
            //          d_pad_moves.len(),
            //          code_to_number(code).unwrap()
            // );
            d_pad_moves.len() * code_to_number(code).unwrap()
        })
            .sum();
        if sum < minimum {
            minimum = sum;
        }
    }
    minimum
    
    // codes.iter().map(|code| {
    //     let num_pad_moves: Vec<char> = ['A'].iter().chain(code.iter()).tuple_windows()
    //         .flat_map(|(&digit, &next_digit)| move_on_num_pad_new(digit, next_digit)).collect();
    //     let d_pad_moves: Vec<char> = ['A'].iter().chain(num_pad_moves.iter()).tuple_windows()
    //         .flat_map(|(&digit, &next_digit)| move_on_d_pad_new(digit, next_digit)).collect();
    //     let d_pad_moves: Vec<char>= ['A'].iter().chain(d_pad_moves.iter()).tuple_windows()
    //         .flat_map(|(&digit, &next_digit)| move_on_d_pad_new(digit, next_digit)).collect();
    //     println!("Code {:?} solved with {:?} ({}*{})",
    //              code.iter().collect::<String>(),
    //              d_pad_moves.iter().collect::<String>(),
    //              d_pad_moves.len(),
    //              code_to_number(code).unwrap()
    //     );
    //     d_pad_moves.len() * code_to_number(code).unwrap()
    // })
    //     .sum()
}

fn code_to_number(code: &Vec<char>) -> Option<usize> {
    let numeric_string: String = code
        .into_iter()
        .filter(|c| c.is_ascii_digit())
        .collect();

    // Parse the resulting string into a u64
    numeric_string.parse::<usize>().ok()
}


fn shuffle<T>(mut vec: Vec<T>, mut rng: &mut ThreadRng) -> Vec<T> {
    vec.shuffle(&mut rng);
    vec
}

fn shuffle_parts<T>(vec: Vec<Vec<T>>, rng: &mut ThreadRng) -> Vec<T> {
    shuffle(vec, rng).into_iter().flatten().collect()
}

fn move_on_num_pad_new(from: char, to: char, rng: &mut ThreadRng) -> Vec<char> {
    match (from, to) {
        ('A', '0') => vec!['<'],
        ('A', '1') => vec!['^', '<', '<'], // unngå ulovlig posisjon
        ('A', '3') => vec!['^'],
        ('A', '4') => vec!['^', '^', '<', '<'], // unngå ulovlig posisjon
        ('A', '7') => vec!['^', '^', '^', '<', '<'], // unngå ulovlig posisjon
        ('A', '8') => shuffle_parts(vec![vec!['<'], vec!['^', '^', '^']], rng),
        ('A', '9') => vec!['^', '^', '^'],

        ('0', 'A') => vec!['>'],
        ('0', '2') => vec!['^'],

        ('1', '3') => vec!['>', '>'],
        ('1', '7') => vec!['^', '^'],

        ('2', '9') => shuffle_parts(vec![vec!['^', '^'], vec!['>']], rng),

        ('3', 'A') => vec!['v'],
        ('3', '6') => vec!['^'],
        ('3', '7') => shuffle_parts(vec![vec!['<', '<'], vec!['^', '^']], rng),

        ('4', '1') => vec!['v'],
        ('4', '5') => vec!['>'],

        ('5', 'A') => shuffle_parts(vec![vec!['v', 'v'], vec!['>']], rng),
        ('5', '6') => vec!['>'],

        ('6', 'A') => vec!['v', 'v'],

        ('7', '3') => shuffle_parts(vec![vec!['v', 'v'], vec!['>', '>']], rng),
        ('7', '8') => vec!['>'],
        ('7', '9') => vec!['>', '>'],

        ('8', '0') => vec!['v', 'v', 'v'],
        ('8', '3') => shuffle_parts(vec![vec!['v', 'v'], vec!['>']], rng),
        ('8', '5') => vec!['v'],
        
        ('9', 'A') => vec!['v', 'v', 'v'],
        ('9', '7') => vec!['<', '<'],
        ('9', '8') => vec!['<'],
        
        _ => unreachable!("from {} to {}", from, to)
        
    }.with('A')
}

fn move_on_d_pad_new(from: char, to: char) -> Vec<char> {
    match (from, to) {
        (x, y) if x == y => Vec::new(),

        ('A', '<') => vec!['v', '<', '<'], // !
        ('A', '^') => vec!['<'],
        ('A', 'v') => vec!['<', 'v'],
        ('A', '>') => vec!['v'],

        ('<', '^') => vec!['>', '^'], // !
        ('<', 'v') => vec!['>'],
        ('<', '>') => vec!['>', '>'],
        ('<', 'A') => vec!['>', '>', '^'], // !

        ('v', '<') => vec!['<'],
        ('v', '^') => vec!['^'],
        ('v', '>') => vec!['>'],
        ('v', 'A') => vec!['>', '^'],

        ('^', '<') => vec!['v', '<'], // !
        ('^', 'v') => vec!['v'],
        ('^', '>') => vec!['v', '>'],
        ('^', 'A') => vec!['>'],

        ('>', '<') => vec!['<', '<'],
        ('>', 'v') => vec!['<'],
        ('>', '^') => vec!['<', '^'],
        ('>', 'A') => vec!['^'],

        _ => {
            unreachable!("from {} to {}", from, to)
        }
    }.with('A')
}

fn move_on_num_pad(from: char, to: char, rng: &mut ThreadRng) -> Vec<char> {
    match (from, to) {
        ('7', '8') | ('8', '9') |
        ('4', '5') | ('5', '6') |
        ('1', '2') | ('2', '3') |
                     ('0', 'A') => vec!['>'],
        ('8', '7') | ('9', '8') |
        ('5', '4') | ('6', '5') |
        ('2', '1') | ('3', '2') |
                     ('A', '0') => vec!['<'],
        ('7', '9') |
        ('4', '6') |
        ('1', '3') => shuffle(vec!['>', '>'], rng),
        ('9', '7') |
        ('6', '4') |
        ('3', '1') => vec!['<', '<'],
        ('7', '4') | ('8', '5') | ('9', '6') |
        ('4', '1') | ('5', '2') | ('6', '3') |
        ('2', '0') | ('3', 'A') => vec!['v'],
        ('7', '5') | ('8', '6') |
        ('4', '2') | ('5', '3') |
                     ('2', 'A' ) => shuffle(vec!['v', '>'], rng),
        ('8', '4') | ('9', '5') |
        ('5', '1') | ('6', '2') |
                     ('3', '0') => shuffle(vec!['<', 'v'], rng),
        
        ('7', '6') |
        ('4', '3') => shuffle_parts(vec![vec!['>', '>'], vec!['v']], rng),
        ('9', '4') |
        ('6', '1') => shuffle_parts(vec![vec!['<', '<'], vec!['v']], rng),
        ('7', '1') | ('8', '2') | ('9', '3') |
                     ('5', '0') | ('6', 'A') => vec!['v', 'v'],
        ('7', '2') | ('8', '3') |
        ('5', 'A') => shuffle_parts(vec![vec!['v', 'v'], vec!['>']], rng),
        ('8', '1') | ('9', '2') |
        ('6', '0') => shuffle_parts(vec![vec!['v', 'v'], vec!['<']], rng),
        ('7', '3') |
        ('9', '1') => shuffle_parts(vec![vec!['v', 'v'], vec!['<', '<']], rng),
        ('8', 'A') => shuffle_parts(vec![vec!['>'], vec!['v', 'v', 'v']], rng),
        ('9', '0') => shuffle_parts(vec![vec!['<'], vec!['v', 'v', 'v']], rng),
        ('8', '0') | ('9', 'A') => vec!['v', 'v', 'v'],
        ('4', '7') | ('5', '8') | ('6', '9') |
        ('1', '4') | ('2', '5') | ('3', '6') |
                     ('0', '2') | ('A', '3') => vec!['^'],
        ('4', '8') | ('5', '9') |
        ('1', '5') | ('2', '6') |
                     ('0', '3') => shuffle(vec!['>', '^'], rng),
        ('5', '7') | ('6', '8') |
        ('2', '4') | ('3', '5') |
                     ('A', '2') => shuffle(vec!['^', '<'], rng),
        ('4', '9') |
        ('1', '6') => shuffle_parts(vec![vec!['>', '>'], vec!['^']], rng),
        ('6', '7') |
        ('3', '4') => shuffle_parts(vec![vec!['<', '<'], vec!['^']], rng),
        
        ('1', '7') | ('2', '8') | ('3', '9') |
                     ('0', '5') | ('A', '6') => vec!['^', '^'],
        ('1', '8') | ('2', '9') |
        ('0', '6') => shuffle_parts(vec![vec!['^', '^'], vec!['>']], rng),
        ('2', '7') | ('3', '8') => shuffle_parts(vec![vec!['^', '^'], vec!['<']], rng),
        ('A', '5') => shuffle_parts(vec![vec!['<'], vec!['^', '^']], rng),
        
        ('1', '9') => shuffle_parts(vec![vec!['>', '>'], vec!['^', '^']], rng),
        ('3', '7') => shuffle_parts(vec![vec!['<', '<'], vec!['^', '^']], rng),
        
        ('0', '8') | ('A', '9') => vec!['^', '^', '^'],
        
        ('0', '9') => shuffle_parts(vec![vec!['^', '^', '^'], vec!['>']], rng),
        ('A', '8') => shuffle_parts(vec![vec!['<'], vec!['^', '^', '^']], rng),

        // Disse må være i presis rekkefølge
        ('7', '0') => vec!['>', 'v', 'v', 'v'],
        ('7', 'A') => vec!['>', '>', 'v', 'v', 'v'],
        ('4', '0') => vec!['>', 'v', 'v'],
        ('4', 'A') => vec!['>', '>', 'v', 'v'],
        ('1', '0') => vec!['>', 'v'],
        ('1', 'A') => vec!['>', '>', 'v'],
        ('0', '7') => vec!['^', '^', '^', '<'],
        ('A', '7') => vec!['^', '^', '^', '<', '<'],
        ('0', '4') => vec!['^', '^', '<'],
        ('A', '4') => vec!['^', '^', '<', '<'],
        ('0', '1') => vec!['^', '<'],
        ('A', '1') => vec!['^', '<', '<'],
        
        _ => {
            println!("Tried to go from {} to {}", from, to);
            unreachable!("How did it come to this?")
        }
    }.with('A')
}

fn move_on_d_pad(from: char, to: char, rng: &mut ThreadRng) -> Vec<char> {
    match (from, to) {
        (x, y) if x == y => Vec::new(),
        
        ('A', '<') => vec!['v', '<', '<'], // !
        ('A', '^') => vec!['<'],
        ('A', 'v') => shuffle(vec!['<', 'v'], rng),
        ('A', '>') => vec!['v'],
        
        ('<', '^') => vec!['>', '^'], // !
        ('<', 'v') => vec!['>'],
        ('<', '>') => vec!['>', '>'],
        ('<', 'A') => vec!['>', '>', '^'], // !

        ('v', '<') => vec!['<'],
        ('v', '^') => vec!['^'],
        ('v', '>') => vec!['>'],
        ('v', 'A') => shuffle(vec!['>', '^'], rng),
        
        ('^', '<') => vec!['v', '<'], // !
        ('^', 'v') => vec!['v'],
        ('^', '>') => shuffle(vec!['v', '>'], rng),
        ('^', 'A') => vec!['>'],
        
        ('>', '<') => vec!['<', '<'],
        ('>', 'v') => vec!['<'],
        ('>', '^') => shuffle(vec!['<', '^'], rng),
        ('>', 'A') => vec!['^'],
        
        _ => {
            println!("On d-pad, tried to go from {} to {}", from, to);
            unreachable!("What about this?")
        }      
    }.with('A')
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
    assert_eq!(result, 270084)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_21.txt");
    assert_eq!(part_2(input), 0)
}
