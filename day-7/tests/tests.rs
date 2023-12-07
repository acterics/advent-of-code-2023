
#[cfg(test)]
mod tests {
    use day_7::task1;
    use day_7::task2;
    
    
    #[test]
    fn test_task1_with_example_input() {
        let result = task1::solution("day-7-example-input.txt");
        assert_eq!(result, 6440)
    }

    #[test]
    fn test_task2_with_example_input() {
        let result = task2::solution("day-7-example-input.txt");
        assert_eq!(result, 5905)
    }
    
}