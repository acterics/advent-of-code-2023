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
            return false
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
