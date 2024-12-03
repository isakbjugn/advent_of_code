use regex::Regex;

pub fn part_1(input: &str) -> u32 {
    let pattern = r"mul\((\d+),(\d+)\)";

    let re = Regex::new(pattern).unwrap();
    let mut sum = 0;

    for caps in re.captures_iter(input) {
        let num1 = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let num2 = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        sum += num1 * num2;
    }
    sum
}

pub fn part_2(input: &str) -> u32 {
    let mul_pattern = r"mul\((\d+),(\d+)\)";
    let do_pattern = r"do\(\)";
    let dont_pattern = r"don't\(\)";

    let mul_re = Regex::new(mul_pattern).unwrap();
    let do_re = Regex::new(do_pattern).unwrap();
    let dont_re = Regex::new(dont_pattern).unwrap();

    let products = mul_re
        .captures_iter(input)
        .map(|capture| {
            let num1 = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let num2 = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
            (capture.get(0).unwrap().start(), num1 * num2)
        })
        .collect::<Vec<(usize, u32)>>();

    let dos = do_re
        .captures_iter(input)
        .map(|capture| capture.get(0).unwrap().start())
        .collect::<Vec<usize>>();
    let donts = dont_re
        .captures_iter(input)
        .map(|capture| capture.get(0).unwrap().start())
        .collect::<Vec<usize>>();

    let mut sum = 0;
    'product: for (index, product) in products.iter() {
        for closest_index in (0..*index).rev() {
            if dos.contains(&closest_index) {
                sum += *product;
                continue 'product;
            }
            if donts.contains(&closest_index) {
                continue 'product;
            }
            if closest_index == 0 {
                sum += *product;
                continue 'product;
            }
        }
    }
    sum
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_3_1.txt");
    assert_eq!(part_1(input), 161)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_3_2.txt");
    assert_eq!(part_2(input), 48)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_3.txt");
    assert_eq!(part_1(input), 173731097)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_3.txt");
    assert_eq!(part_2(input), 93729253)
}
