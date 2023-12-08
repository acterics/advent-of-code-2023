pub const LEFT_COMMAND: char = 'L';
pub const RIGHT_COMMAND: char = 'R';

pub struct NetworkConnection<'a> {
    pub source: &'a str,
    pub left_dest: &'a str,
    pub right_dest: &'a str,
}

impl<'a> NetworkConnection<'a> {
    pub fn parse_network_connection(line: &'a str) -> (&'a str, NetworkConnection<'a>) {
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

        return (
            source,
            NetworkConnection {
                source,
                left_dest,
                right_dest,
            },
        );
    }
}
