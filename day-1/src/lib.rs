
pub mod task1 {
    use std::fs;
    
    pub fn solution(filename: &str) -> i32 {
        let file_content = fs::read_to_string(filename)
            .expect("Failed to open {filename}");
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
