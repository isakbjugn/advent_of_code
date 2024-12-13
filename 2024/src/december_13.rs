use rayon::prelude::*;
use regex::Regex;

pub fn part_1(input: &str) -> u64 {
    let machines: Vec<Machine> = input
        .split("\n\n")
        .map(to_machine)
        .map(|machine_option| machine_option.expect("Could not create machine from str"))
        .collect();

    machines
        .into_par_iter()
        .filter_map(|machine| machine.tokens_to_win(100))
        .sum()
}

fn to_machine(machine_str: &str) -> Option<Machine> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    if let Some(captures) = re.captures(machine_str) {
        let a_x = captures.get(1)?.as_str().parse::<i64>().ok()?;
        let a_y = captures.get(2)?.as_str().parse::<i64>().ok()?;
        let b_x = captures.get(3)?.as_str().parse::<i64>().ok()?;
        let b_y = captures.get(4)?.as_str().parse::<i64>().ok()?;
        let prize_x = captures.get(5)?.as_str().parse::<i64>().ok()?;
        let prize_y = captures.get(6)?.as_str().parse::<i64>().ok()?;

        return Some(Machine {
            a: (a_x, a_y),
            b: (b_x, b_y),
            prize: (prize_x, prize_y),
        });
    }
    None
}

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    pub fn tokens_to_win(&self, max_tokens: i64) -> Option<u64> {
        match self.solve_equation_system(self.a.0, self.b.0, self.prize.0, self.a.1, self.b.1, self.prize.1) {
            Some((x, y)) if x < 0 || y < 0 => None,
            Some((x, y)) if max_tokens > 0 && (x > max_tokens || y > max_tokens) => None,
            Some((x, y)) => Some(3 * x as u64 + y as u64),
            None => None,
        }
    }

    fn solve_equation_system(&self, a_0: i64, b_0: i64, c_0: i64, a_1: i64, b_1: i64, c_1: i64) -> Option<(i64, i64)> {
        let x = match (b_1 * c_0 - b_0 * c_1) as f64 / (b_1 * a_0 - b_0 * a_1) as f64 {
            float if float.fract() != 0.0 => return None,
            integer => integer as i64,
        };

        let y = match (c_0 - a_0 * x) as f64 / b_0 as f64 {
            float if float.fract() != 0.0 => return None,
            integer => integer as i64,
        };

        match a_0 * x + b_0 * y == c_0 {
            true => Some((x, y)),
            false => None,
        }
    }
}

pub fn part_2(input: &str) -> u64 {
    let machines: Vec<Machine> = input
        .split("\n\n")
        .map(to_machine)
        .map(|machine_option| machine_option.expect("Could not create machine from str"))
        .map(|machine| Machine { a: machine.a, b: machine.b, prize: (machine.prize.0 + 10000000000000, machine.prize.1 + 10000000000000) })
        .collect();

    machines
        .into_par_iter()
        .filter_map(|machine| machine.tokens_to_win(0))
        .sum()
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_13.txt");
    assert_eq!(part_1(input), 480)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_13.txt");
    assert_eq!(part_2(input), 875318608908)
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_13.txt");
    assert_eq!(part_1(input), 29023)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_13.txt");
    assert_eq!(part_2(input), 96787395375634)
}
