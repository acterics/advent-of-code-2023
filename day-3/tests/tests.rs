#[cfg(test)]
mod tests {

    use day_3::task1;
    
    #[test]
    fn test_task1_with_example_input() {
        let result = task1::solution("task-1-example-input.txt");
        assert_eq!(result, 4361);
    }
}