use std::fs;

use crate::common::{Position, START, neighbour_tiles, has_connections, connected_tiles};

pub fn solution(filename: &str) -> i32 {
    let file_content = fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
    let rows = Vec::from_iter(file_content.split('\n'));
    let start = find_start_position(&rows);
    
    let loop_size = get_loop_size(&start, &rows);
    return loop_size / 2;
}

pub fn get_loop_size<'a>(start: &'a Position, rows: &Vec<&str>) -> i32 {
    let loop_start = get_start_loop_tile(&start, &rows);
    
    let Some(loop_size) = _get_loop_size(start,  &start, &loop_start, rows, 1) else {
        panic!("Loop not found")
    };
    return loop_size;
}

fn _get_loop_size(start: &Position, previous: &Position, current: &Position, rows: &Vec<&str>, step: i32) -> Option<i32> {
    let Some(tile) = get_tile(current, rows) else {
        panic!("Cannot get tile ({}, {})", current.x, current.y)
    };
    let connected_tiles = connected_tiles(current, tile);
    for next_tile in connected_tiles {
        if next_tile == *previous {
           continue; 
        }
        if next_tile == *start {
            return Some(step + 1)
        }
        return _get_loop_size(start,  current, &next_tile, rows, step + 1)
    }
    return None;
}

pub fn get_start_loop_tile(start: &Position, rows: &Vec<&str>) -> Position {
    let start_neighbours = neighbour_tiles(&start);
    for start_neighbour in start_neighbours {
        let Some(tile) = get_tile(&start_neighbour, &rows) else {
            continue;
        };
        if !has_connections(tile) {
            continue;
        }
        let connected = connected_tiles(&start_neighbour, tile);
        if connected[0] == *start {
            return start_neighbour
        }
    }
    panic!("Loop start not found");
}

pub fn get_tile(position: &Position, rows: &Vec<&str>) -> Option<char> {
    if position.y < 0 || position.x < 0 {
        return None
    }
    let Some(row) = rows.get(position.y as usize) else {
        return None;
    };
    return row.chars().nth(position.x as usize)
    
}

pub fn find_start_position(rows: &Vec<&str>) -> Position {
    for (row_index, row) in rows.iter().enumerate() {
        for (tile_index, tile) in row.chars().enumerate() {
            if tile == START  {
                return Position { x: tile_index as i32, y: row_index as i32 }
            }
        }
    }
    panic!("Start position not found")
}