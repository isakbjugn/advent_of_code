use std::collections::{HashSet, VecDeque};
use dashmap::DashMap;
use rayon::prelude::*;

pub fn part_1(input: &str) -> u64 {
    input.par_lines()
        .map(|line| line.parse::<u64>().expect("Secret number must parse to int"))
        .map(|secret_number| {
            let mut number = secret_number;
            for _ in 0..2000 {
                number = generate(number);
            }
            number
        })
        .sum()
}

fn generate(number: u64) -> u64 {
    let first_mixer = number * 64;
    let number = mix_and_prune(number, first_mixer);
    let second_mixer = number / 32;
    let number = mix_and_prune(number, second_mixer);
    let third_mixer = number * 2048;
    mix_and_prune(number, third_mixer)
}

fn mix_and_prune(number: u64, mixer: u64) -> u64 {
    (number ^ mixer) % 16777216
}

pub fn part_2(input: &str) -> (Vec<i8>, u16) {
    let all_sequences_and_prices: DashMap<Vec<i8>, u16> = DashMap::new();
    input.par_lines()
        .map(|line| line.parse::<u64>().expect("Secret number must parse to int"))
        .for_each(|secret_number| {
            let mut number = secret_number;
            let mut price_changes = VecDeque::new();
            let mut last_price = None;
            let mut seen: HashSet<Vec<i8>> = HashSet::new();
            for _ in 0..2000 {
                number = generate(number);
                let price = last_digit(number);
                if let Some(last_price) = last_price {
                    let change = price as i8 - last_price as i8;
                    if price_changes.len() == 4 {
                        price_changes.pop_front();
                    }
                    price_changes.push_back(change);
                    if price_changes.len() == 4 {
                        let sequence_key: Vec<i8> = price_changes.iter().cloned().collect();
                        if !seen.contains(&sequence_key) {
                            if let Some(mut current_price) = all_sequences_and_prices.get_mut(&sequence_key) {
                                *current_price += price;
                            } else {
                                all_sequences_and_prices.insert(sequence_key.clone(), price);
                            }
                        }
                        seen.insert(sequence_key);
                    }
                }
                last_price = Some(price);                
            }
        });
    
    all_sequences_and_prices
        .into_iter()
        .max_by(|kv_pair, rhs_kv_pair| kv_pair.1.cmp(&rhs_kv_pair.1))
        .expect("There must be a max!")
}

fn last_digit(number: u64) -> u16 {
    (number % 10) as u16
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_22_1.txt");
    assert_eq!(part_1(input), 37327623)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_22_2.txt");
    assert_eq!(part_2(input), (vec![-2, 1, -1, 3], 23))
}

#[test]
fn input_part_1() {
    let input = include_str!("../input/input_22.txt");
    assert_eq!(part_1(input), 17163502021)
}

#[test]
fn input_part_2() {
    let input = include_str!("../input/input_22.txt");
    assert_eq!(part_2(input), (vec![1, 0, -2, 2], 1938))
}
