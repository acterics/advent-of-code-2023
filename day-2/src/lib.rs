pub mod task1 {
    use std::fs;

    const MAX_RED_COUNT: i32 = 12;
    const MAX_GREEN_COUNT: i32 = 13;
    const MAX_BLUE_COUNT: i32 = 14;

    const RED_TOKEN: &str = "red";
    const GREEN_TOKEN: &str = "green";
    const BLUE_TOKEN: &str = "blue";

    struct Game {
        id: i32,
        sets: Vec<GameSet>,
    }

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

    struct GameSet {
        red: i32,
        green: i32,
        blue: i32,
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
            .map(|line| parse_line(line))
            .filter(|game| game.is_valid())
            .fold(0, |acc, game| acc + game.id);

        return result;
    }

    fn parse_line(line: &str) -> Game {
        let mut game_parts = line.split(':');

        let raw_game_info = game_parts.nth(0).unwrap();
        let game_id = parse_game_id(raw_game_info);

        let raw_game_sets = game_parts.nth(0).unwrap();

        let game_sets = raw_game_sets.split(';').map(|raw| parse_game_set(raw));

        let game_sets_vec = Vec::from_iter(game_sets);

        return Game {
            id: game_id,
            sets: game_sets_vec,
        };
    }

    fn parse_game_set(raw: &str) -> GameSet {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let parts = raw.split(',');
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

    fn parse_game_id(raw: &str) -> i32 {
        return raw
            .split(' ')
            .nth(1)
            .unwrap()
            .parse()
            .expect(&format!("Failed to get game_id from {raw}"));
    }
}
