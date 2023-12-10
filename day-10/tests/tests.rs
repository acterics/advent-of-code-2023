#[cfg(test)]
mod tests {
    use day_10::task1;

    #[test]
    fn test_task1_with_example_input_1() {
        let result = task1::solution("day-10-example-input-1.txt");
        assert_eq!(result, 4)
    }

    #[test]
    fn test_task1_with_example_input_2() {
        let result = task1::solution("day-10-example-input-2.txt");
        assert_eq!(result, 8)
    }
}