pub fn part_1(input: &str) -> i32 {
    input.lines()
        .map(to_card)
        .map(|card| card.calculate_points())
        .sum()
}

#[derive(Clone, Debug)]
struct Card {
    number: i16,
    winning: Vec<i8>,
    scratched: Vec<i8>,
}

impl Card {
    fn get_num_winning(&self) -> usize {
        self.scratched
            .iter()
            .filter(|&num| self.winning.contains(&num))
            .count()
    }
    fn calculate_points(&self) -> i32 {
        match self.get_num_winning() {
            0 => 0,
            n => 2_i32.pow(n as u32 - 1)
        }
    }
    fn get_new_cards(&self) -> Option<Vec<i16>> {
        match self.get_num_winning() {
            0 => None,
            n => Some((self.number+1..self.number+1+n as i16).collect())
        }
    }
}

fn to_card(input: &str) -> Card {
    let (card_str, numbers) = input.split_once(':').unwrap();
    let card_num = card_str.split_once(' ').unwrap().1.trim();
    let lists = to_two_lists(numbers);

    Card { number: card_num.parse::<i16>().unwrap(), winning: lists.0, scratched: lists.1 }
}

fn to_two_lists(input: &str) -> (Vec<i8>, Vec<i8>) {
    let lists: Vec<Vec<i8>> = input
        .split('|')
        .map(|number_str| number_str
            .split_whitespace()
            .map(|str| str.parse::<i8>().unwrap())
            .collect())
        .collect();
    (lists.get(0).unwrap().to_vec(), lists.get(1).unwrap().to_vec())
}

pub fn part_2(input: &str) -> i32 {
    let mut cards_scratched = 0;
    let cards: Vec<Card> = input.lines().map(to_card).collect();
    let mut cards_to_scratch = Vec::<Card>::new();

    for card in cards.iter() {
        cards_to_scratch.push(card.clone());
    }

    while !cards_to_scratch.is_empty() {
        let card = cards_to_scratch.pop().unwrap();
        cards_scratched += 1;
        if let Some(new_cards) = card.get_new_cards() {
            for new_card_number in new_cards {
                cards_to_scratch.push(cards.get(new_card_number as usize - 1).unwrap().clone());
            }
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
fn sample_input_part_2_1() {
    let input = include_str!("../input/sample_4.txt");
    assert_eq!(part_2(input), 30)
}
