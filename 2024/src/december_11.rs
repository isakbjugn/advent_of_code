
pub fn part_1(input: &str) -> usize {
    let mut line: Vec<u64> = input.split_whitespace()
        .map(|c| c.parse::<u64>().expect("Could not input to integers"))
        .collect();
    
    let mut new_line = Vec::<u64>::new();
    for _ in 0..25 {
        new_line = Vec::<u64>::new();
        for stone in line {
            match stone {
                0 => new_line.push(1),
                number if has_even_digits(number) => {
                    if let Some((first, second)) = split_even_digits(number) {
                        new_line.push(first);
                        new_line.push(second);
                    } else { panic!() }
                },
                number => new_line.push(number * 2024)
            }
        }
        line = new_line;
        // println!("{:?}", line);
    }
    line.len()
}

fn has_even_digits(number: u64) -> bool {
    if number == 0 {
        return false; // 0 has 1 digit, which is odd
    }

    let digit_count = number.ilog10() + 1;
    digit_count % 2 == 0
}

fn split_even_digits(n: u64) -> Option<(u64, u64)> {
    if n == 0 {
        return None; // Edge case: 0 has only one digit, which is odd
    }

    let digit_count = n.ilog10() + 1;

    // Return None if the number of digits is odd
    if digit_count % 2 != 0 {
        return None;
    }

    let half_digits = digit_count / 2;
    let divisor = 10_u64.pow(half_digits);

    // Split the number into two parts
    let first_half = n / divisor;
    let second_half = n % divisor;

    Some((first_half, second_half))
}

pub fn part_2(input: &str) -> usize {
    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_11.txt");
    assert_eq!(part_1(input), 55312)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_11.txt");
    assert_eq!(part_2(input), 0)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_11.txt");
    assert_eq!(part_1(input), 222461)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_11.txt");
    assert_eq!(part_2(input), 0)
}
