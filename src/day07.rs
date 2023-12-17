use crate::utils;
use std::cmp::Ordering;

const LABELS: &str = "23456789TJQKA";

#[derive(Clone, Copy)]
struct CamelCard {
    label: char,
}

#[derive(Eq)]
struct Hand {
    cards: [CamelCard; 5],
}

impl CamelCard {
    pub fn strength(&self) -> u32 {
        if self.label.is_ascii_digit() {
            return self.label.to_digit(10).unwrap();
        } else {
            for (i, letter) in "TJQKA".chars().enumerate() {
                if self.label == letter {
                    return (i + 10) as u32;
                }
            }
        }
        0
    }
}

impl PartialEq for CamelCard {
    fn eq(&self, other: &CamelCard) -> bool {
        self.label == other.label
    }
}

impl Eq for CamelCard {}

impl Hand {
    fn get_all(&self) -> String {
        let mut all_hand: String = String::new();

        for card in &self.cards {
            all_hand.push(card.label);
        }

        all_hand
    }

    pub fn set_cards(&mut self, cards: &str) {
        for (i, card) in cards.chars().enumerate() {
            self.cards[i].label = card;
        }
    }
    pub fn strength(&self) -> u32 {
        let mut strength = 0;
        for label in LABELS.chars() {
            if self.get_all().find(label).is_some() {
                let count = self.get_all().matches(label).count();
                strength += match count {
                    5 => 8,
                    4 => 7,
                    3 => 4,
                    2 => 1,
                    _ => 0,
                }
            }
        }

        strength
    }

    pub fn new() -> Self {
        let card = CamelCard { label: '2' };
        Self { cards: [card; 5] }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut cmp = self.strength().cmp(&other.strength());
        let mut i = 0;
        if cmp == Ordering::Equal {
            while cmp == Ordering::Equal {
                cmp = self.cards[i].strength().cmp(&other.cards[i].strength());
                i += 1;
            }
        }
        cmp
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.strength() == other.strength()
    }
}

pub fn main() {
    let input = utils::string_from_file("07.txt");

    let mut hands_bids: Vec<(Hand, u32)> = Vec::new();

    for line in input.lines() {
        let (cards, bid) = line.split_at(line.find(' ').unwrap());
        let mut hand = Hand::new();
        hand.set_cards(cards);
        hands_bids.push((hand, bid.trim().parse::<u32>().unwrap()));
    }
    hands_bids.sort();

    let mut result = 0;
    for (i, (_hand, bid)) in hands_bids.iter().enumerate() {
        result += bid * (i as u32 + 1);
    }
    println!("{}", result);
}
