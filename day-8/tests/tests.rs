#[cfg(test)]
mod tests {
    use day_8::task1;


    #[test]
    fn test_task1_with_example_input_1() {
        let result = task1::solution("day-8-example-input-1.txt");
        assert_eq!(result, 2)
    }

    #[test]
    fn test_task1_with_example_input_2() {
        let result = task1::solution("day-8-example-input-2.txt");
        assert_eq!(result, 6)
    }

}