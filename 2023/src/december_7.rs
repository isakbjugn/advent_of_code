use std::cmp::Ordering;
use itertools::Itertools;

pub fn part_1(input: &str) -> i32 {
    input.lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(card_str, bid)| (Hand::new(card_str), bid.parse::<i32>().unwrap()))
        .sorted_by(|(hand_1, _), (hand_2, _)| hand_1.partial_cmp(hand_2).unwrap())
        .enumerate()
        .map(|(index, (_, bid))| (index as i32 + 1) * bid )
        .sum()
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(card_str: &str) -> Hand {
        Hand {
            cards: card_str.chars().map(Card::new).collect()
        }
    }
    fn get_type(&self) -> HandType {
        if self.cards.iter().all_equal() {
            HandType::FiveOfAKind
        } else if self.most_frequent_card().1 == 4 {
            HandType::FourOfAKind
        } else if self.most_frequent_card().1 == 3 && self.second_most_frequent_card().1 == 2 {
            HandType::FullHouse
        } else if self.most_frequent_card().1 == 3 && self.cards.iter().unique().count() == 3 {
            HandType::ThreeOfAKind
        } else if self.most_frequent_card().1 == 2 && self.second_most_frequent_card().1 == 2 && self.cards.iter().unique().count() == 3 {
            HandType::TwoPairs
        } else if self.most_frequent_card().1 == 2 && self.second_most_frequent_card().1 == 1 && self.cards.iter().unique().count() == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
    fn most_frequent_card(&self) -> (Option<&Card>, usize) {
        let mut most_frequent_card = None;
        let mut most_frequent_card_count = 0;
        for card in self.cards.iter().unique() {
            let card_count = self.cards.iter().filter(|&c| c == card).count();
            if card_count > most_frequent_card_count {
                most_frequent_card = Some(card);
                most_frequent_card_count = card_count;
            }
        }
        (most_frequent_card, most_frequent_card_count)
    }
    fn second_most_frequent_card(&self) -> (Option<&Card>, usize) {
        let (most_frequent_card_opt, count) = self.most_frequent_card();
        match count {
            5 => (None, 0),
            _ => {
                let most_frequent_card = most_frequent_card_opt.unwrap();
                let mut second_most_frequent_card = None;
                let mut second_most_frequent_card_count = 0;
                for card in self.cards.iter().unique() {
                    let card_count = self.cards.iter().filter(|&c| c == card).count();
                    if card_count > second_most_frequent_card_count && card != most_frequent_card {
                        second_most_frequent_card = Some(card);
                        second_most_frequent_card_count = card_count;
                    }
                }
                (second_most_frequent_card, second_most_frequent_card_count)
            }
        }
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.get_type() != other.get_type() { return false }
        for card in 0..5 {
            if self.cards[card] != other.cards[card] { return false }
        }
        true
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.get_type() != other.get_type() { return self.get_type().partial_cmp(&other.get_type()) }
        for card in 0..5 {
            if self.cards[card] != other.cards[card] { return self.cards[card].partial_cmp(&other.cards[card]) }
        }
        Some(Ordering::Equal)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack, // 11
    Queen, // 12
    King, // 13
    Ace, // 14
}

impl Card {
    pub fn new(card: char) -> Card {
        match card {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {}", card)
        }
    }
}

#[derive(PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn part_2(input: &str) -> i16 {

    0
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_7.txt");
    assert_eq!(part_1(input), 6440)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_7.txt");
    assert_eq!(part_2(input), 0)
}