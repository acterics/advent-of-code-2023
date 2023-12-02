pub mod task1 {
    use crate::common::Game;
    use crate::common::GameSet;
    use std::fs;

    const MAX_RED_COUNT: i32 = 12;
    const MAX_GREEN_COUNT: i32 = 13;
    const MAX_BLUE_COUNT: i32 = 14;

    impl Game {
        fn is_valid(&self) -> bool {
            for set in self.sets.iter() {
                if !set.is_valid() {
                    return false;
                }
            }
            return true;
        }
    }

    impl GameSet {
        fn is_valid(&self) -> bool {
            return self.red <= MAX_RED_COUNT
                && self.green <= MAX_GREEN_COUNT
                && self.blue <= MAX_BLUE_COUNT;
        }
    }

    pub fn solution(filename: &str) -> i32 {
        let file_content =
            fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
        let lines = file_content.split('\n');

        let result = lines
            .map(|line| Game::parse_game(line))
            .filter(|game| game.is_valid())
            .fold(0, |acc, game| acc + game.id);

        return result;
    }

}

pub mod task2 {
    use crate::common::Game;
    use crate::common::GameSet;
    use std::cmp;
    use std::fs;

    impl Game {
        fn required_game_set(&self) -> GameSet {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for set in &self.sets {
                max_red = cmp::max(set.red, max_red);
                max_green = cmp::max(set.green, max_green);
                max_blue = cmp::max(set.blue, max_blue);
            }
            
            return GameSet {red: max_red, green: max_green, blue: max_blue}
        }
    }

    impl GameSet {
        fn power(&self) -> i64 {
            return (self.red as i64) * (self.green as i64) * (self.blue as i64);
        }
    }


    pub fn solution(filename: &str) -> i64 {
        let file_content =
            fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
        let lines = file_content.split('\n');

        let result = lines
            .map(|line| Game::parse_game(line).required_game_set().power())
            .fold(0, |acc, power| acc + power);

        return result;
    }
}

mod common {
    const RED_TOKEN: &str = "red";
    const GREEN_TOKEN: &str = "green";
    const BLUE_TOKEN: &str = "blue";
    pub struct Game {
        pub id: i32,
        pub sets: Vec<GameSet>,
    }

    pub struct GameSet {
        pub red: i32,
        pub green: i32,
        pub blue: i32,
    }

    impl Game {
        pub fn parse_game(str: &str) -> Game {
            let mut game_parts = str.split(':');

            let raw_game_info = game_parts.nth(0).unwrap();
            let game_id = Game::parse_game_id(raw_game_info);

            let raw_game_sets = game_parts.nth(0).unwrap();

            let game_sets = raw_game_sets
                .split(';')
                .map(|raw| GameSet::parse_game_set(raw));

            let game_sets_vec = Vec::from_iter(game_sets);

            return Game {
                id: game_id,
                sets: game_sets_vec,
            };
        }

        fn parse_game_id(str: &str) -> i32 {
            return str
                .split(' ')
                .nth(1)
                .unwrap()
                .parse()
                .expect(&format!("Failed to get game_id from {str}"));
        }
    }

    impl GameSet {
        fn parse_game_set(str: &str) -> GameSet {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            let parts = str.split(',');
            for raw_color_result in parts {
                let mut raw_color_parts = raw_color_result.trim().split(' ');
                let count: i32 = raw_color_parts
                    .nth(0)
                    .unwrap()
                    .parse()
                    .expect(&format!("Failed to parse {raw_color_result}"));
                let color = raw_color_parts.nth(0).unwrap();
                match color {
                    RED_TOKEN => red = count,
                    GREEN_TOKEN => green = count,
                    BLUE_TOKEN => blue = count,
                    _ => panic!("Unknown color token {color}"),
                }
            }
            return GameSet { red, green, blue };
        }
    }
}
