pub mod task1 {
    use crate::common::Hand;
    use std::fs;

    pub fn solution(filename: &str) -> i32 {
        let file_content =
            fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
        let hands_iterator = file_content.split('\n').map(|line| Hand::parse_hand(line));
        let mut hands = Vec::from_iter(hands_iterator);
        hands.sort();
        
        let result = hands
            .iter()
            .enumerate()
            .fold(0, |acc, (index, hand)| acc + hand.bid * (index as i32 + 1));

        return result;
    }
}

mod common {
    use once_cell::sync::Lazy;
    use std::{cmp::Ordering, collections::HashMap, sync::Mutex};

    static CARD_RANKS: Lazy<Mutex<HashMap<char, i32>>> = Lazy::new(|| {
        let map = HashMap::from([
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('J', 11),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
        ]);
        return Mutex::new(map);
    });

    pub struct Hand<'a> {
        pub cards: &'a str,
        hand_type: HandType,
        pub bid: i32,
    }

    impl<'a> Hand<'a> {
        pub fn parse_hand(line: &'a str) -> Hand<'a> {
            let mut split = line.split(' ');
            let cards = split
                .next()
                .expect(&format!("Failed to get cards from {}", &line));
            let bid: i32 = split
                .next()
                .expect(&format!("Failed to get bid from {}", &line))
                .parse()
                .expect(&format!("Failed to parse bid from {}", &line));
            let hand_type = Hand::get_hand_type(cards);
            return Hand { cards, hand_type, bid };
        }
    }

    impl<'a> Ord for Hand<'a> {
        fn cmp(&self, other: &Self) -> Ordering {
            let hand_type_cmp = self.hand_type.cmp(&other.hand_type);
            
            if hand_type_cmp.is_ne() {
                return hand_type_cmp
            }
            for (card, other_card) in self.cards.chars().zip(other.cards.chars()) {
                let card_cmp = compare_cards(&card, &other_card);
                if card_cmp.is_ne() {
                    return card_cmp
                }
            }
            
            return Ordering::Equal;
        }
    }

    impl<'a> PartialEq for Hand<'a> {
        fn eq(&self, other: &Self) -> bool {
            self.cards == other.cards
        }
    }

    impl<'a> Eq for Hand<'a> {}

    impl<'a> PartialOrd for Hand<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    fn compare_cards(card1: &char, card2: &char) -> Ordering {
        let ranks = CARD_RANKS.lock().unwrap();
        return ranks[card1].cmp(&ranks[card2]);
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum HandType {
        HighCard,
        Pair,
        TwoPairs,
        Three,
        FullHouse,
        Four,
        Five,
    }

    impl <'a> Hand<'a> {
        fn get_hand_type(cards: &'a str) -> HandType {
            let mut duplicated_cards: HashMap<char, i32> = HashMap::new();
            for card in cards.chars() {
                if duplicated_cards.contains_key(&card) {
                    duplicated_cards.insert(card, duplicated_cards[&card] + 1);
                } else {
                    duplicated_cards.insert(card, 1);
                }
            }
            if duplicated_cards.len() == 5 {
                return HandType::HighCard
            }
            
            if duplicated_cards.len() == 4 {
                return HandType::Pair
            }
            
            if duplicated_cards.len() == 1 {
                return HandType::Five
            }
            
            let (_, max_count) = duplicated_cards.iter()
                    .max_by_key(|(_, count)| *count).unwrap();
            
            if duplicated_cards.len() == 3 {
                if *max_count == 3 {
                    return HandType::Three
                }
                if *max_count == 2 {
                    return HandType::TwoPairs
                }
                panic!("Unknown hand type {cards}")
            }
            if duplicated_cards.len() == 2 {
                if *max_count == 4 {
                    return HandType::Four
                }
                if *max_count == 3 {
                    return HandType::FullHouse
                }
                panic!("Unknown hand type {cards}")
            }
            
            panic!("Unknown hand type {cards}")
        }
    }
}
