use std::{collections::HashMap, fs};
use num::integer::lcm;

use crate::common::{NetworkConnection, LEFT_COMMAND, RIGHT_COMMAND};

const START_TOKEN: char = 'A';
const FINAL_TOKEN: char = 'Z';

pub fn solution(filename: &str) -> i64 {
    let file_content = fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
    let mut lines = file_content.split('\n');
    let commands = lines.next().unwrap();
    lines.next();

    let mut initial_locations: Vec<&str> = Vec::new();

    let connections: HashMap<&str, NetworkConnection> = HashMap::from_iter(
        lines
            .map(|line| NetworkConnection::parse_network_connection(line))
            .inspect(|(source, _)| {
                if source.ends_with(START_TOKEN) {
                    initial_locations.push(source);
                }
            }),
    );

    let steps = get_required_steps_count(commands, &connections, &mut initial_locations);

    return steps;
}

fn get_required_steps_count<'a>(
    commands: &'a str,
    connections: &'a HashMap<&'a str, NetworkConnection>,
    current_locations: &Vec<&str>,
) -> i64 {
    let mut periods: Vec<i64> = Vec::with_capacity(current_locations.len());
    for location in current_locations.into_iter() {
        let mut current_location = *location;
        let mut step = 0;
        while !current_location.ends_with(FINAL_TOKEN) {
            for command in commands.chars() {
                let connection = connections.get(current_location).unwrap();
                let new_location: &str;
                match command {
                    LEFT_COMMAND => {
                        new_location = connection.left_dest;
                    }
                    RIGHT_COMMAND => {
                        new_location = connection.right_dest;
                    }
                    _ => panic!("Unknown command ({command}) from ({commands})"),
                }
                current_location = new_location;
                step += 1;
                if current_location.ends_with(FINAL_TOKEN) {
                    break;
                }
            }
        }
        periods.push(step);
    }
    
    let result = periods.iter().fold(1, |acc, period| {
        // Lowest Common Multiple (LCM)
        lcm(acc, *period)
    });
    
    return result;
}