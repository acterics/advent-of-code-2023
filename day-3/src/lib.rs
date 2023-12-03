pub mod task1 {
    use std::{fs, ops::AddAssign};

    const DOT_TOKEN: char = '.';

    pub fn solution(filename: &str) -> i32 {
        let file_content =
            fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
        let mut lines_iterator = file_content.split('\n');
        let mut previous_line: Option<&str> = None;
        let mut current_line: Option<&str> = lines_iterator.next();
        let mut next_line: Option<&str> = lines_iterator.next();
        let mut sum = 0;
        loop {
            if current_line.is_none() {
                return 0;
            }

            let line_result = process_line(current_line.unwrap(), previous_line, next_line);
            sum += line_result;

            if next_line.is_none() {
                break;
            }

            previous_line = current_line;
            current_line = next_line;
            next_line = lines_iterator.next();
        }

        return sum;
    }

    fn process_line(line: &str, previous_line: Option<&str>, next_line: Option<&str>) -> i32 {
        let mut sum = 0;

        let mut digit_chars: Vec<char> = Vec::with_capacity(line.len());
        let mut number_start_index: i32 = 0;
        let mut char_before_digits: Option<char> = None;

        let mut line_iterator = line.chars();

        struct State {
            previous_char: Option<char>,
            current_char: Option<char>,
            next_char: Option<char>,
            i: i32,
        }

        let mut state = State {
            previous_char: None,
            current_char: line_iterator.next(),
            next_char: line_iterator.next(),
            i: 0,
        };

        let mut iterate = |state: &mut State| {
            state.previous_char = state.current_char;
            state.current_char = state.next_char;
            state.next_char = line_iterator.next();
            state.i += 1;
        };

        let update_sum = |digits: &mut Vec<char>, sum: &mut i32| {
            let number_string = String::from_iter(digits.iter());
            let number: i32 = number_string
                .parse()
                .expect(&format!("Failed to parse string {number_string}"));
            sum.add_assign(number);
            digits.clear();
        };

        loop {
            if let Some(char) = state.current_char {
                if char.is_digit(10) {
                    if digit_chars.is_empty() {
                        char_before_digits = state.previous_char;
                        number_start_index = state.i;
                    }
                    digit_chars.push(char);
                    iterate(&mut state);
                    continue;
                }
            }

            if !digit_chars.is_empty() {
                if let Some(previous_char) = char_before_digits {
                    if previous_char != DOT_TOKEN {
                        update_sum(&mut digit_chars, &mut sum);
                        iterate(&mut state);
                        continue;
                    }
                }
                if let Some(next_number_char) = state.current_char {
                    if next_number_char != DOT_TOKEN {
                        update_sum(&mut digit_chars, &mut sum);
                        iterate(&mut state);
                        continue;
                    }
                }

                if let Some(previous_line) = previous_line {
                    if is_number_with_neighbour_symbol(
                        number_start_index,
                        digit_chars.len(),
                        previous_line,
                    ) {
                        update_sum(&mut digit_chars, &mut sum);
                        iterate(&mut state);
                        continue;
                    }
                }
                if let Some(next_line) = next_line {
                    if is_number_with_neighbour_symbol(
                        number_start_index,
                        digit_chars.len(),
                        next_line,
                    ) {
                        update_sum(&mut digit_chars, &mut sum);
                        iterate(&mut state);
                        continue;
                    }
                }
                digit_chars.clear();
            }

            if state.current_char.is_none() {
                break;
            } else {
                iterate(&mut state);
            }
        }

        return sum;
    }

    fn is_number_with_neighbour_symbol(index: i32, size: usize, line: &str) -> bool {
        if line.is_empty() {
            return false;
        }
        let mut i: i32 = index;
        let line_bytes = line.as_bytes();
        if index != 0 {
            i -= 1;
        }
        while i <= index + size as i32 {
            if i == line.len() as i32 {
                break;
            }
            let char = line_bytes[i as usize] as char;
            if char != DOT_TOKEN {
                return true;
            };

            i += 1;
        }
        return false;
    }
}

pub mod task2 {
    use std::fs;

    const GEAR_TOKEN: char = '*';
    const GEAR_PARTS: usize = 2;

    pub fn solution(filename: &str) -> i32 {
        let file_content =
            fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
        let mut lines_iterator = file_content.split('\n');
        let mut previous_line: Option<&str> = None;
        let mut current_line: Option<&str> = lines_iterator.next();
        let mut next_line: Option<&str> = lines_iterator.next();
        let mut sum = 0;
        loop {
            if current_line.is_none() {
                return 0;
            }

            let line_result = process_line(current_line.unwrap(), previous_line, next_line);
            sum += line_result;

            if next_line.is_none() {
                break;
            }

            previous_line = current_line;
            current_line = next_line;
            next_line = lines_iterator.next();
        }

        return sum;
    }

    fn process_line(line: &str, previous_line: Option<&str>, next_line: Option<&str>) -> i32 {
        let mut sum = 0;
        for (index, char) in line.char_indices() {
            if char == GEAR_TOKEN {
                if let Some(gear_ratio) = get_gear_ratio(index, line, previous_line, next_line) {
                    sum += gear_ratio;
                }
            }
        }
        return sum;
    }

    fn get_gear_ratio(
        index: usize,
        line: &str,
        previous_line: Option<&str>,
        next_line: Option<&str>,
    ) -> Option<i32> {
        let mut parts: Vec<i32> = Vec::new();
        if let Some(digits) = get_gear_part_from_left(index, line) {
            parts.push(parse_digits(digits));
        }
        if let Some(digits) = get_gear_part_from_right(index, line) {
            parts.push(parse_digits(digits));
        }
        if let Some(previous_line) = previous_line {
            let top_adjust_numbers = get_adjust_numbers(index, previous_line);
            if parts.len() + top_adjust_numbers.len() > GEAR_PARTS {
                return None;
            }
            parts.extend(top_adjust_numbers);
        }
        if let Some(next_line) = next_line {
            let bottom_adjust_numbers = get_adjust_numbers(index, next_line);
            if parts.len() + bottom_adjust_numbers.len() > GEAR_PARTS {
                return None;
            }
            parts.extend(bottom_adjust_numbers);
        }

        if parts.len() != GEAR_PARTS {
            return None;
        }
        return Some(parts[0] * parts[1]);
    }

    fn get_gear_part_from_left(index: usize, line: &str) -> Option<Vec<char>> {
        if index == 0 {
            return None;
        }
        let line_bytes = line.as_bytes();
        let prev_char = line_bytes[index - 1] as char;
        if !prev_char.is_digit(10) {
            return None;
        }
        let mut i = index - 1;
        let mut digits: Vec<char> = Vec::new();

        loop {
            let char = line_bytes[i] as char;
            if !char.is_digit(10) {
                break;
            }
            digits.insert(0, char);
            if i == 0 {
                break
            } else {
                i -= 1;
            }
        }
        return Some(digits)
    }

    fn get_gear_part_from_right(index: usize, line: &str) -> Option<Vec<char>> {
        if index == line.len() - 1 {
            return None;
        }
        let line_bytes = line.as_bytes();
        let next_char = line_bytes[index + 1] as char;
        if !next_char.is_digit(10) {
            return None;
        }
        let mut i = index + 1;
        let mut digits: Vec<char> = Vec::new();
        while i != line.len() {
            let char = line_bytes[i] as char;
            if !char.is_digit(10) {
                break;
            }
            digits.push(char);
            i += 1;
        }
        return Some(digits)
    }

    fn get_adjust_numbers(index: usize, line: &str) -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::new();
        let line_bytes = line.as_bytes();

        if (line_bytes[index] as char).is_digit(10) {
            let mut digits: Vec<char> = Vec::new();
            if let Some(left_digits) = get_gear_part_from_left(index, line) {
                digits.extend(left_digits);
            }
            digits.push(line_bytes[index] as char);

            if let Some(right_digits) = get_gear_part_from_right(index, line) {
                digits.extend(right_digits);
            }
            let number = parse_digits(digits);
            vec.push(number);
            return vec
        }

        if let Some(left_digits) = get_gear_part_from_left(index, line) {
            vec.push(parse_digits(left_digits));
        }
        if let Some(right_digits) = get_gear_part_from_right(index, line) {
            vec.push(parse_digits(right_digits))
        }
        return vec;
    }

    fn parse_digits(digits: Vec<char>) -> i32 {
        let number_string = String::from_iter(&digits);
        return number_string.parse()
            .expect(&format!("Failed to parse {number_string}"))
    }
}
