pub fn is_all_numbers_zero(numbers: &[i32]) -> bool {
    for number in numbers {
        if *number != 0 {
            return false
        }
    }
    return true;
}
