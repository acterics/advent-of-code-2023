use std::collections::HashMap;
use std::cmp::Ordering;

pub struct Hand<'a> {
    pub cards: &'a str,
    pub hand_type: HandType,
    pub bid: i32,
}

impl<'a> Hand<'a> {
    pub fn parse_hand(line: &'a str, hand_type_fn: fn(&str) -> HandType) -> Hand<'a> {
        let mut split = line.split(' ');
        let cards = split
            .next()
            .expect(&format!("Failed to get cards from {}", &line));
        let bid: i32 = split
            .next()
            .expect(&format!("Failed to get bid from {}", &line))
            .parse()
            .expect(&format!("Failed to parse bid from {}", &line));
        let hand_type = hand_type_fn(cards);
        return Hand {
            cards,
            hand_type,
            bid,
        };
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl<'a> Eq for Hand<'a> {}

pub fn compare_cards(card1: &char, card2: &char, ranks: &HashMap<char, i32>) -> Ordering {
    return ranks[card1].cmp(&ranks[card2]);
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPairs,
    Three,
    FullHouse,
    Four,
    Five,
}
