
pub const NORTH_SOUTH: char = '|';
pub const EAST_WEST: char = '-';
pub const NORTH_EAST: char = 'L';
pub const NORTH_WEST: char = 'J';
pub const SOUTH_WEST: char = '7';
pub const SOUTH_EAST: char = 'F';
pub const GROUND: char = '.';
pub const START: char = 'S';

pub struct Node {
    pub pos: Position,
    pub tile: char
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

pub fn has_connections(tile: char) -> bool {
    match tile {
        GROUND => { return false }
        START => { return false }
        _ => { return true }
    }
}

pub fn neighbour_tiles(position: &Position) -> [Position; 4] {
    return [
        Position { x: position.x, y: position.y - 1 },
        Position { x: position.x - 1, y: position.y },
        Position { x: position.x, y: position.y + 1 },
        Position { x: position.x + 1, y: position.y }
    ]
}

pub fn connected_tiles(position: &Position, tile: char) -> [Position; 2] {
    let diff1: (i32, i32);
    let diff2: (i32, i32);
    match tile {
        NORTH_SOUTH => {
            diff1 = (0, -1);
            diff2 = (0, 1);
        }
        EAST_WEST => {
            diff1 = (-1, 0);
            diff2 = (1, 0);
        }
        NORTH_EAST => {
            diff1 = (0, -1);
            diff2 = (1, 0);
        }
        NORTH_WEST => {
            diff1 = (-1, 0);
            diff2 = (0, -1);
        }
        SOUTH_EAST => {
            diff1 = (1, 0);
            diff2 = (0, 1);
        }
        SOUTH_WEST => {
            diff1 = (0, 1);
            diff2 = (-1, 0)
        }
        
        _ => panic!("Tile ({tile}) without connections")
    }
    return [
        Position { x: position.x + diff1.0, y: position.y + diff1.1 },
        Position { x: position.x + diff2.0, y: position.y + diff2.1 }
    ]
}
