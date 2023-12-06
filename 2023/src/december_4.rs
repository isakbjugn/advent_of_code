use std::collections::{HashSet, HashMap};

pub fn part_1(input: &str) -> i32 {
    input.lines()
        .map(to_card)
        .map(|card| card.calculate_points())
        .sum()
}

#[derive(Clone, Debug)]
struct Card {
    number: i16,
    winning: HashSet<i8>,
    scratched: HashSet<i8>,
}

impl Card {
    fn get_num_winning(&self) -> usize {
        self.scratched.intersection(&self.winning).count()
    }
    fn calculate_points(&self) -> i32 {
        match self.get_num_winning() {
            0 => 0,
            n => 2_i32.pow(n as u32 - 1)
        }
    }
    fn get_new_cards(&self) -> Vec<i16> {
        match self.get_num_winning() {
            0 => Vec::new(),
            n => (self.number+1..self.number+1+n as i16).collect()
        }
    }
}

fn to_card(input: &str) -> Card {
    let (card_str, numbers) = input.split_once(':').unwrap();
    let card_num = card_str.split_once(' ').unwrap().1.trim();
    let lists = to_two_sets(numbers);

    Card { number: card_num.parse::<i16>().unwrap(), winning: lists.0, scratched: lists.1 }
}

fn to_two_sets(input: &str) -> (HashSet<i8>, HashSet<i8>) {
    let lists: Vec<HashSet<i8>> = input
        .split('|')
        .map(|number_str| number_str
            .split_whitespace()
            .map(|str| str.parse::<i8>().unwrap())
            .collect())
        .collect();
    (HashSet::from_iter(lists.get(0).unwrap().iter().cloned()), HashSet::from_iter(lists.get(1).unwrap().iter().cloned()))
}

pub fn part_2(input: &str) -> i32 {
    let mut cards_scratched = 0;
    let cards: Vec<Card> = input.lines().map(to_card).collect();
    let card_to_new_cards: HashMap<i16, Vec<i16>> = cards
        .iter()
        .map(|card| (card.number, card.get_new_cards())).collect();
    let mut cards_to_scratch = Vec::<i16>::new();

    for card in cards.iter() {
        cards_to_scratch.push(card.number);
    }

    while let Some(card_number) = cards_to_scratch.pop() {
        cards_scratched += 1;
        for new_card_number in card_to_new_cards.get_key_value(&card_number).unwrap().1 {
            cards_to_scratch.push(*new_card_number);
        }
    }
    cards_scratched
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_4.txt");
    assert_eq!(part_1(input), 13)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_4.txt");
    assert_eq!(part_2(input), 30)
}
