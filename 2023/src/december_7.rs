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

pub fn part_2(input: &str) -> i32 {
    input.lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(card_str, bid)| (Hand::new_with_joker(card_str), bid.parse::<i32>().unwrap()))
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
    pub fn new_with_joker(card_str: &str) -> Hand {
        Hand {
            cards: card_str.chars().map(Card::new).map(|card| match card {
                Card::Jack => Card::Joker,
                _ => card
            }).collect()
        }
    }
    fn get_type(&self) -> HandType {
        match self.highest_frequency() + self.jokers() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 if self.second_highest_frequency() == 2 => HandType::FullHouse,
            3 => HandType::ThreeOfAKind,
            2 if self.second_highest_frequency() == 2 => HandType::TwoPair,
            2 if self.second_highest_frequency() == 1 => HandType::OnePair,
            1 => HandType::HighCard,
            _ => panic!()
        }
    }
    fn count(&self) -> Vec<(&Card, usize)> {
        self.cards.iter()
            .map(|card| (card, self.cards.iter().filter(|&c| c == card && *card != Card::Joker).count()))
            .unique()
            .sorted_by(|(_, count_1), (_, count_2)| count_2.cmp(count_1))
            .collect()
    }
    fn highest_frequency(&self) -> usize {
        self.count().get(0).unwrap().1
    }
    fn second_highest_frequency(&self) -> usize {
        self.count().get(1).unwrap().1
    }
    fn jokers(&self) -> usize {
        self.cards.iter().filter(|&card| *card == Card::Joker).count()
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

#[derive(PartialEq, PartialOrd, Eq, Hash, Debug)]
enum Card {
    Joker, // Verdt minst som enkeltkort
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

#[derive(PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_7.txt");
    assert_eq!(part_1(input), 6440)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_7.txt");
    assert_eq!(part_2(input), 5905)
}

#[test]
fn actual_input_part_1() {
    let input = include_str!("../input/input_7.txt");
    assert_eq!(part_1(input), 246912307)
}

#[test]
fn actual_input_part_2() {
    let input = include_str!("../input/input_7.txt");
    assert_eq!(part_2(input), 246894760)
}

#[test]
fn five_of_a_kind() {
    let hand = Hand::new("KKKKK");
    assert_eq!(hand.get_type(), HandType::FiveOfAKind)
}

#[test]
fn five_of_a_kind_with_jokers() {
    let hand_with_one_joker = Hand::new_with_joker("KKKJK");
    assert_eq!(hand_with_one_joker.get_type(), HandType::FiveOfAKind);
    let hand_with_two_jokers = Hand::new_with_joker("KKKJJ");
    assert_eq!(hand_with_two_jokers.get_type(), HandType::FiveOfAKind);
    let hand_with_three_jokers = Hand::new_with_joker("KKJJJ");
    assert_eq!(hand_with_three_jokers.get_type(), HandType::FiveOfAKind);
    let hand_with_four_jokers = Hand::new_with_joker("KJJJJ");
    assert_eq!(hand_with_four_jokers.get_type(), HandType::FiveOfAKind);
    let hand_with_five_jokers = Hand::new_with_joker("JJJJJ");
    assert_eq!(hand_with_five_jokers.get_type(), HandType::FiveOfAKind)
}

#[test]
fn four_of_a_kind() {
    let hand = Hand::new("K2KKK");
    assert_eq!(hand.get_type(), HandType::FourOfAKind)
}

#[test]
fn four_of_a_kind_with_jokers() {
    let hand_with_one_joker = Hand::new_with_joker("K2KJK");
    assert_eq!(hand_with_one_joker.get_type(), HandType::FourOfAKind);
    let hand_with_two_jokers = Hand::new_with_joker("K2KJJ");
    assert_eq!(hand_with_two_jokers.get_type(), HandType::FourOfAKind);
    let hand_with_three_jokers = Hand::new_with_joker("K2JJJ");
    assert_eq!(hand_with_three_jokers.get_type(), HandType::FourOfAKind);

    // Endrer type ved tilstrekkelig antall jokere
    let hand_with_four_jokers = Hand::new_with_joker("KJJJJ");
    assert_eq!(hand_with_four_jokers.get_type(), HandType::FiveOfAKind)
}

#[test]
fn full_house() {
    let hand = Hand::new("K22KK");
    assert_eq!(hand.get_type(), HandType::FullHouse)
}

#[test]
fn full_house_with_jokers() {
    let hand_with_one_joker = Hand::new_with_joker("K22JK");
    assert_eq!(hand_with_one_joker.get_type(), HandType::FullHouse);

    // Endrer type ved tilstrekkelig antall jokere
    let hand_with_two_jokers = Hand::new_with_joker("K22JJ");
    assert_eq!(hand_with_two_jokers.get_type(), HandType::FourOfAKind)
}

#[test]
fn three_of_a_kind() {
    let hand = Hand::new("K29KK");
    assert_eq!(hand.get_type(), HandType::ThreeOfAKind)
}

#[test]
fn three_of_a_kind_with_jokers() {
    let hand_with_one_joker = Hand::new_with_joker("K29JK");
    assert_eq!(hand_with_one_joker.get_type(), HandType::ThreeOfAKind);
    let hand_with_two_jokers = Hand::new_with_joker("K29JJ");
    assert_eq!(hand_with_two_jokers.get_type(), HandType::ThreeOfAKind);
}

#[test]
fn two_pair() {
    let hand = Hand::new("KK229");
    assert_eq!(hand.get_type(), HandType::TwoPair)
}

#[test]
fn two_pair_with_joker() {
    // Får aldri to par med jokere
    let hand_with_one_joker = Hand::new_with_joker("KK2J9");
    assert_eq!(hand_with_one_joker.get_type(), HandType::ThreeOfAKind);
    let hand_with_two_jokers = Hand::new_with_joker("K2JJ9");
    assert_eq!(hand_with_two_jokers.get_type(), HandType::ThreeOfAKind)
}

#[test]
fn one_pair() {
    let hand = Hand::new("KK229");
    assert_eq!(hand.get_type(), HandType::TwoPair)
}

#[test]
fn one_pair_with_joker() {
    let hand_with_one_joker = Hand::new_with_joker("K234J");
    assert_eq!(hand_with_one_joker.get_type(), HandType::OnePair);
}

#[test]
fn high_card() {
    let hand = Hand::new("K2345");
    assert_eq!(hand.get_type(), HandType::HighCard)
}

#[test]
fn high_card_with_joker() {
    // Får aldri høyt kort med joker
    let hand_with_one_joker = Hand::new_with_joker("K234J");
    assert_eq!(hand_with_one_joker.get_type(), HandType::OnePair);
}