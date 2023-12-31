pub mod task1 {
    use std::fs;
    use crate::common::get_winning_numbers_count;

    pub fn solution(filename: &str) -> i32 {
        let file_content =
            fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
        let lines = file_content.split('\n');

        let result = lines
            .map(|line| get_card_points(line))
            .fold(0, |acc, points| acc + points);

        return result;
    }

    fn get_card_points(card_str: &str) -> i32 {
        let winning_game_numbers_count = get_winning_numbers_count(card_str);

        if winning_game_numbers_count == 0 {
            return 0;
        };
        let base: i32 = 2;
        return base.pow((winning_game_numbers_count - 1) as u32);
    }
}

pub mod task2 {
    use std::fs;
    use crate::common::get_winning_numbers_count;

    pub fn solution(filename: &str) -> u32 {
        let file_content =
            fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
        let lines = file_content.split('\n');

        let mut cards: Vec<u32> = Vec::new();

        let mut count: usize = 0;
        for line in lines {
            process_card(line, count, &mut cards);
            count += 1;
        }

        cards.truncate(count);

        return cards.iter().take(count)
            .fold(0, |acc, number| { acc + number })
    }

    fn process_card(card_str: &str, index: usize, cards: &mut Vec<u32>) {
        let winning_count = get_winning_numbers_count(card_str);
        let current_count: u32;
        if let Some(count) = cards.get(index) {
            current_count = count + 1;
            cards[index] = current_count;
        } else {
            current_count = 1;
            cards.push(current_count);
        }
        for copy_index in (index + 1)..(index + 1 + winning_count) {
            if let Some(count) = cards.get(copy_index) {
                cards[copy_index] = count + current_count
            } else {
                cards.push(current_count);
            }
        }
    }

}

mod common {
    const SEPARATOR_TOKEN: char = '|';

    pub fn get_winning_numbers_count(card_str: &str) -> usize {
        let mut card_split = card_str.split(':');
        let _raw_card_name = card_split
            .next()
            .expect(&format!("Cannot get card name from {card_str}"));
        let raw_card_numbers = card_split
            .next()
            .expect(&format!("Cannot get card numbers from {card_str}"));

        let mut raw_card_numbers_split = raw_card_numbers.split(SEPARATOR_TOKEN);
        let raw_winning_numbers = raw_card_numbers_split
            .next()
            .expect(&format!("Cannot get winning numbers from {card_str}"));
        let raw_game_numbers = raw_card_numbers_split
            .next()
            .expect(&format!("Cannot get game numbers from {card_str}"));

        let winning_numbers_iterator = raw_winning_numbers
            .trim()
            .split(' ')
            .filter(|element| !element.is_empty())
            .map(|raw_number| -> i32 {
                return raw_number.parse().expect(&format!(
                    "Cannot get winning number {raw_number} from {card_str}"
                ));
            });

        let winning_numbers = Vec::from_iter(winning_numbers_iterator);

        let game_numbers = raw_game_numbers
            .trim()
            .split(' ')
            .filter(|element| !element.is_empty())
            .map(|raw_number| -> i32 {
                return raw_number.parse().expect(&format!(
                    "Cannot get game number {raw_number} from {card_str}"
                ));
            });

        let winning_game_numbers_count = game_numbers
            .filter(|number| winning_numbers.contains(number))
            .count();
        return winning_game_numbers_count;
    }
}