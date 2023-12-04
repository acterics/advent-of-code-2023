#[cfg(test)]
mod tests {

    use day_4::task1;

    #[test]
    fn test_task1_with_example_input() {
        let result = task1::solution("day-4-example-input.txt");
        assert_eq!(result, 13);
    }
}