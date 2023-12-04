#[cfg(test)]
mod tests {

    use day_4::task1;
    use day_4::task2;

    #[test]
    fn test_task1_with_example_input() {
        let result = task1::solution("day-4-example-input.txt");
        assert_eq!(result, 13);
    }

    #[test]
    fn test_task2_with_example_input() {
        let result = task2::solution("day-4-example-input.txt");
        assert_eq!(result, 30);
    }
}