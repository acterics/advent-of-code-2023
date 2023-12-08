use std::{collections::HashMap, fs};

use crate::common::{NetworkConnection, LEFT_COMMAND, RIGHT_COMMAND};

const START_LOCATION: &str = "AAA";
const FINAL_LOCATION: &str = "ZZZ";

pub fn solution(filename: &str) -> i32 {
    let file_content = fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
    let mut lines = file_content.split('\n');
    let commands = lines.next().unwrap();
    lines.next();
    let connections: HashMap<&str, NetworkConnection> =
        HashMap::from_iter(lines.map(|line| NetworkConnection::parse_network_connection(line)));

    let steps = get_required_steps_count(commands, &connections);

    return steps;
}

fn get_required_steps_count(commands: &str, connections: &HashMap<&str, NetworkConnection>) -> i32 {
    let mut step = 0;
    let mut current_location = START_LOCATION;
    while current_location != FINAL_LOCATION {
        for command in commands.chars() {
            if current_location == FINAL_LOCATION {
                return step;
            }
            let connection = connections.get(current_location).unwrap();
            match command {
                LEFT_COMMAND => {
                    current_location = connection.left_dest;
                }
                RIGHT_COMMAND => {
                    current_location = connection.right_dest;
                }
                _ => panic!("Unknown command ({command}) from ({commands})"),
            }
            step += 1;
        }
    }
    return step;
}
