use crate::utils;

struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Clone for Card {
    fn clone(&self) -> Card {
        Card {
            number: self.number,
            winning_numbers: self.winning_numbers.clone(),
            numbers: self.numbers.clone(),
        }
    }
}

pub fn main() {
    let input = utils::string_from_file("04.txt");

    let mut result = 0;
    let mut cards: Vec<Card> = Vec::new();
    for line in input.lines() {
        cards.push(line_into_card(line));
    }

    let new_cards: Vec<Card> = cards.clone();
    result += add_cards(&cards, new_cards);

    println!("{}", result);
}

fn add_cards(all_cards: &Vec<Card>, new_cards: Vec<Card>) -> u32 {
    let mut points: u32 = 0;
    points += new_cards.len() as u32;
    for card in new_cards.iter() {
        let matches = matches_by_card(card.to_owned());

        if matches > 0 {
            let copy_cards: Vec<Card> = Vec::from_iter(
                all_cards[(card.number) as usize..card.number as usize + matches as usize].to_vec(),
            );

            points += add_cards(all_cards, copy_cards);
        }
    }
    points
}
fn matches_by_card(card: Card) -> u32 {
    let mut matches = 0;
    for number in card.numbers.iter() {
        for winning_number in card.winning_numbers.iter() {
            if number == winning_number {
                matches += 1;
            }
        }
    }

    matches
}
fn line_into_card(line: &str) -> Card {
    let mut card = Card {
        number: 0,
        winning_numbers: Vec::new(),
        numbers: Vec::new(),
    };

    let sides = line.split('|').collect::<Vec<&str>>();
    let left_side = sides[0].split(':').collect::<Vec<&str>>();

    card.number = left_side[0]
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();

    for number in left_side[1]
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()
    {
        card.winning_numbers.push(number.parse::<u32>().unwrap());
    }

    for number in sides[1]
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()
    {
        card.numbers.push(number.parse::<u32>().unwrap());
    }
    card
}
