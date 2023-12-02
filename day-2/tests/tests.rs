#[cfg(test)]
mod tests {

    use day_2::task1;
    use day_2::task2;
    
    #[test]
    fn test_task1_with_example_input() {
        let result = task1::solution("day-2-example-input.txt");
        assert_eq!(result, 8);
    }

    #[test]
    fn test_task2_with_example_input() {
        let result = task2::solution("day-2-example-input.txt");
        assert_eq!(result, 2286);
    }
}