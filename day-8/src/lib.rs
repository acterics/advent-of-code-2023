pub mod task1 {
    use std::{fs, collections::HashMap};

    const START_LOCATION: &str = "AAA";
    const FINAL_LOCATION: &str = "ZZZ";
    const LEFT_COMMAND: char = 'L';
    const RIGHT_COMMAND: char = 'R';
     
    
    pub fn solution(filename: &str) -> i32 {
        let file_content =
            fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
        let mut lines = file_content.split('\n');
        let commands = lines.next().unwrap();
        lines.next();
        let connections: HashMap<&str, NetworkConnection> =
            HashMap::from_iter(lines.map(|line| parse_network_connection(line)));
        
        let steps = get_required_steps_count(commands, &connections);
        
        return steps
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
                    _ => panic!("Unknown command ({command}) from ({commands})")
                }
                step += 1;
            }
        }
        return step;
    }

    fn parse_network_connection<'a>(line: &'a str) -> (&'a str, NetworkConnection<'a>) {
        let mut line_split = line.split('=');
        let source = line_split.next().unwrap().trim();
        let mut destinations_split = line_split
            .next()
            .and_then(|str| str.trim().strip_prefix('('))
            .and_then(|str| str.strip_suffix(')'))
            .expect(&format!("Failed to parse destinations from {line}"))
            .split(',');
        let left_dest = destinations_split.next().unwrap().trim();
        let right_dest = destinations_split.next().unwrap().trim();

        return (source,  NetworkConnection {
            source,
            left_dest,
            right_dest,
        })
    }

    struct NetworkConnection<'a> {
        source: &'a str,
        left_dest: &'a str,
        right_dest: &'a str,
    }
}
