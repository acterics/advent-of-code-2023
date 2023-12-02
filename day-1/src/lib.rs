pub mod task1 {
    use std::fs;

    pub fn solution(filename: &str) -> i32 {
        let file_content = fs::read_to_string(filename).expect("Failed to open {filename}");
        let lines = file_content.split('\n');
        let mut sum = 0;
        for line in lines {
            sum += get_line_result(line)
        }
        return sum;
    }

    fn get_line_result(line: &str) -> i32 {
        let mut i = 0;
        let mut digit_chars: [char; 2] = ['0', '0'];

        for symbol in line.chars() {
            if symbol.is_digit(10) {
                if i == 2 {
                    i -= 1;
                }
                digit_chars[i] = symbol;
                i += 1;
            }
        }
        if i == 1 {
            digit_chars[1] = digit_chars[0];
        }

        return String::from_iter(digit_chars)
            .parse()
            .expect("Failed to parse line {line}");
    }
}

pub mod task2 {
    use std::fs;

    const TEXT_DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    pub fn solution(filename: &str) -> i32 {
        let file_content = fs::read_to_string(filename).expect("Failed to open {filename}");
        let lines = file_content.split('\n');
        let mut sum = 0;
        for line in lines {
            sum += get_line_result(line)
        }
        return sum;
    }

    fn get_line_result(line: &str) -> i32 {
        let mut iteration: usize = 0;
        let mut digit_chars: [char; 2] = ['0', '0'];

        let mut update_digit_state = |symbol: char| {
            if iteration == 2 {
                iteration -= 1;
            }
            digit_chars[iteration] = symbol;
            iteration += 1;
        };

        for (symbol_index, symbol) in line.chars().enumerate() {
            if symbol.is_digit(10) {
                update_digit_state(symbol);
            } else if let Some(digit_symbol) = find_text_digit(line, symbol_index) {
                update_digit_state(digit_symbol);
            }
        }

        if iteration == 1 {
            digit_chars[1] = digit_chars[0];
        }

        return String::from_iter(digit_chars)
            .parse()
            .expect("Failed to parse line {line}");
    }

    fn find_text_digit(line: &str, index: usize) -> Option<char> {
        let mut digit_index: usize = 0;
        for digit in TEXT_DIGITS {
            let mut digit_symbol_index = 0;
            while digit_symbol_index < digit.len() {
                if digit.chars().nth(digit_symbol_index)
                    == line.chars().nth(index + digit_symbol_index)
                {
                    digit_symbol_index += 1
                } else {
                    break;
                }
            }

            if digit_symbol_index == digit.len() {
                let digit_symbol = get_text_digit_value(digit_index);
                return Some(digit_symbol);
            }

            digit_index += 1;
        }
        return None;
    }

    fn get_text_digit_value(digit_index: usize) -> char {
        let offset = '0' as u8;
        return (((digit_index + 1) as u8) + offset) as char;
    }
}
