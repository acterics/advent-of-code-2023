use std::fs;

pub fn solution(filename: &str) -> i32 {
    let file_content = fs::read_to_string(filename).expect(&format!("Failed to open {filename}"));
    let result = file_content
        .split('\n')
        .map(|line| {
            let iterator = line
                .split(' ')
                .map(|str| -> i32 { str.trim().parse().unwrap() });
            return Vec::from_iter(iterator);
        })
        .map(|numbers| predict_next(&numbers))
        .fold(0, |acc, number| acc + number);
    return result;
}

fn predict_next(numbers: &[i32]) -> i32 {
    if numbers.is_empty() {
        panic!("Empty numbers")
    }
    if is_all_numbers_zero(numbers) {
        return 0
    }
    
    let last = numbers.last().unwrap();
    let mut diffs: Vec<i32> = Vec::with_capacity(numbers.len() - 1);
    let mut current: i32 = numbers[0];
    for index in 1..numbers.len() {
        let number = numbers[index];
        diffs.push(number - current);
        current = number
    }
    
    return last + predict_next(&diffs);
}

fn is_all_numbers_zero(numbers: &[i32]) -> bool {
    for number in numbers {
        if *number != 0 {
            return false
        }
    }
    return true;
}
