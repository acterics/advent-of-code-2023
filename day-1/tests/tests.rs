#[cfg(test)]
mod tests {
    use day_1::task1;
    use day_1::task2;

    #[test]
    fn test_task1_with_example_input() {
        let result = task1::solution("task-1-example-input.txt");
        assert_eq!(result, 142)
    }

    #[test]
    fn test_task2_with_example_input() {
        let result = task2::solution("task-2-example-input.txt");
        assert_eq!(result, 281);
    }

    #[test]
    fn test_char_converting() {
        let digit_index: usize = 8;
        let foo = '0' as u8;
        let digit_symbol = (((digit_index + 1) as u8) + foo) as char;
        assert_eq!(digit_symbol, '9');
    }
}
