pub mod task1 {
    use crate::common::Race;
    use std::fs;

    pub fn solution(filename: &str) -> i64 {
        let file_content =
            fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
        let mut lines = file_content.split('\n');
        let time_line = lines.next().expect("Failed to get Time line from input");
        let duration_line = lines
            .next()
            .expect("Failed to get Duration line from input");
        let races = Race::parse_races(time_line, duration_line);

        return races
            .iter()
            .map(|race| race.possible_wins_count())
            .fold(1, |acc, count| acc * count);
    }

    impl Race {
        fn parse_races(time_line: &str, distance_line: &str) -> Vec<Race> {
            let time_line_races_values = time_line
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .split(' ')
                .filter(|str| !str.is_empty())
                .map(|str| -> i64 {
                    str.trim()
                        .parse()
                        .expect(&format!("Failed to parse {str} from {time_line}"))
                });

            let distance_line_races_values = distance_line
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .split(' ')
                .filter(|str| !str.is_empty())
                .map(|str| -> i64 {
                    str.trim()
                        .parse()
                        .expect(&format!("Failed to parse {str} from {distance_line}"))
                });

            let races_iterator = time_line_races_values.zip(distance_line_races_values).map(
                |(duration, distance)| Race {
                    duration_ms: duration,
                    record_distance_mm: distance,
                },
            );

            return Vec::from_iter(races_iterator);
        }
    }
}

pub mod task2 {
    use crate::common::Race;
    use std::fs;

    // Obviously not optimal solution
    pub fn solution(filename: &str) -> i64 {
        let file_content =
            fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
        let mut lines = file_content.split('\n');
        let time_line = lines.next().expect("Failed to get Time line from input");
        let duration_line = lines
            .next()
            .expect("Failed to get Duration line from input");

        let race = Race::parse_race(time_line, duration_line);
        return race.possible_wins_count();
    }

    impl Race {
        fn parse_race(time_line: &str, distance_line: &str) -> Race {
            let time_line_value = time_line
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .split(' ')
                .filter(|str| !str.is_empty())
                .map(|str| str.trim());

            let distance_line_value = distance_line
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .split(' ')
                .filter(|str| !str.is_empty())
                .map(|str| str.trim());

            let duration: i64 = String::from_iter(time_line_value)
                .parse()
                .expect(&format!("Failed to parse duration"));
            let distance: i64 = String::from_iter(distance_line_value)
                .parse()
                .expect(&format!("Failed to parse duration"));
            return Race {
                duration_ms: duration,
                record_distance_mm: distance,
            };
        }
    }
}

mod common {
    pub struct Race {
        pub duration_ms: i64,
        pub record_distance_mm: i64,
    }

    impl Race {
        pub fn predict_result(&self, press_duration: i64) -> i64 {
            let moving_duration = self.duration_ms - press_duration;
            if moving_duration <= 0 {
                return 0;
            }
            let velocity = press_duration;
            return moving_duration * velocity;
        }

        pub fn possible_wins_count(&self) -> i64 {
            // TODO: optimize
            return (0..self.duration_ms)
                .filter(|prees_duration| {
                    return self.predict_result(*prees_duration) > self.record_distance_mm;
                })
                .count() as i64;
        }
    }
}
