
pub fn part_1(input: &str) -> usize {
    let mut file_id_vec = Vec::<String>::new();
    let mut file_id = 0;
    let mut is_file = true;

    for c in input.chars() {
        // println!("{}", c);
        match (is_file, c.to_digit(10)) {
            (true, Some(0)) => unreachable!("File should never be zero blocks"),
            (true, Some(size)) => {
                for _ in 0..size {
                    file_id_vec.push(file_id.to_string())
                }
                file_id += 1;
            },
            (false, Some(0)) => (),
            (false, Some(size)) => {
                for _ in 0..size {
                    file_id_vec.push(String::from("."))
                }
            },
            (_, None) => break,
        }
        is_file = !is_file;
    }
    // println!("{:?}", file_id_vec.concat());
    
    let mut left_index = 0;
    let mut right_index = file_id_vec.len() - 1;
    
    'outer: loop {
        if left_index >= right_index { break };
        
        if file_id_vec.get(left_index) == Some(&".".to_string()) {
            while file_id_vec.get(right_index) == Some(&".".to_string()) {
                right_index -= 1;
                if left_index >= right_index {
                    break 'outer;
                }
            }
            file_id_vec.swap(left_index, right_index)
        }
        left_index += 1;
    }
    // println!("{:?}", file_id_vec.concat());
    
    file_id_vec.into_iter()
        .enumerate()
        .filter(|(_, id)| id != &String::from("."))
        .map(|(index, id)| index * id.parse::<usize>().expect("Could not parse back to number"))
        .sum()
}

pub fn part_2(input: &str) -> usize {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_9.txt");
    assert_eq!(part_1(input), 1928)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_9.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_9.txt");
    assert_eq!(part_1(input), 6367087064415)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_9.txt");
    assert_eq!(part_2(input), 0)
}
