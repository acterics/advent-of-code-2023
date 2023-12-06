#[cfg(test)]
mod tests {

    use day_6::task1;

    #[test]
    fn test_task1_with_example_input() {
        let result = task1::solution("day-6-example-input.txt");
        assert_eq!(result, 288)
    }
}