use once_cell::sync::Lazy;
use std::{cmp::Ordering, collections::HashMap, sync::Mutex, fs};

#[derive(PartialEq, Eq)]
struct Task2Hand<'a> {
    hand: Hand<'a>
}

use crate::common::{Hand, compare_cards, HandType};
pub fn solution(filename: &str) -> i32 {
    let file_content = fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
    let hands_iterator = file_content.split('\n')
        .map(|line| {
            Task2Hand { hand: Hand::parse_hand(line, get_hand_type) }
        });
    let mut hands = Vec::from_iter(hands_iterator);
    hands.sort();

    let result = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (index, task_hand)| {
            acc + task_hand.hand.bid * (index as i32 + 1)
        });

    return result;
}

static CARD_RANKS: Lazy<Mutex<HashMap<char, i32>>> = Lazy::new(|| {
    let map = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
        ]);
    return Mutex::new(map);
});

impl<'a> Ord for Task2Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_cmp = self.hand.hand_type.cmp(&other.hand.hand_type);

        if hand_type_cmp.is_ne() {
            return hand_type_cmp;
        }
        for (card, other_card) in self.hand.cards.chars().zip(other.hand.cards.chars()) {
            let card_cmp = compare_cards(&card, &other_card, &CARD_RANKS.lock().unwrap());
            if card_cmp.is_ne() {
                return card_cmp;
            }
        }

        return Ordering::Equal;
    }
}

impl<'a> PartialOrd for Task2Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn get_hand_type<'a>(cards: &'a str) -> HandType {
    let mut duplicated_cards: HashMap<char, i32> = HashMap::new();
    for card in cards.chars() {
        *duplicated_cards.entry(card).or_insert(0) += 1;
    }
    if duplicated_cards.contains_key(&'J') {
        let jokers_count = duplicated_cards.remove(&'J').unwrap();
        if jokers_count == 5 {
            return HandType::Five
        }
        for count in duplicated_cards.values_mut() {
            *count += jokers_count;
        }
    }
    
    if duplicated_cards.len() == 5 {
        return HandType::HighCard;
    }

    if duplicated_cards.len() == 4 {
        return HandType::Pair;
    }

    if duplicated_cards.len() == 1 {
        return HandType::Five;
    }

    let (_, max_count) = duplicated_cards
            .iter()
            .max_by_key(|(_, count)| *count)
            .unwrap();

    if duplicated_cards.len() == 3 {
        if *max_count == 3 {
            return HandType::Three;
        }
        if *max_count == 2 {
            return HandType::TwoPairs;
        }
        panic!("Unknown hand type {cards}")
    }
    if duplicated_cards.len() == 2 {
        if *max_count == 4 {
            return HandType::Four;
        }
        if *max_count == 3 {
            return HandType::FullHouse;
        }
        panic!("Unknown hand type {cards}")
    }

    panic!("Unknown hand type {cards}")
}