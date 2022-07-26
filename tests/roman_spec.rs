#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(1, "I")]
    #[test_case(10, "X")]
    #[test_case(100, "C")]
    #[test_case(1000, "M")]
    fn roman_power_of_ten_is_single_numeral(number: i32, numeral: &str) {
        assert_eq!(roman_rust::numeral_for(number), numeral);
    }

    #[test_case(1, "I")]
    #[test_case(10, "X")]
    #[test_case(100, "C")]
    #[test_case(1000, "M")]
    fn roman_twice_power_of_ten_is_twice_numeral(number: i32, numeral: &str) {
        assert_eq!(roman_rust::numeral_for(2*number), format!("{0}{0}", numeral));
    }

    #[test_case(1, "I")]
    #[test_case(10, "X")]
    #[test_case(100, "C")]
    #[test_case(1000, "M")]
    fn roman_thrice_power_of_ten_is_thrice_numeral(number: i32, numeral: &str) {
        assert_eq!(roman_rust::numeral_for(3*number), format!("{0}{0}{0}", numeral));
    }


    #[test_case(1, "V")]
    #[test_case(10, "L")]
    #[test_case(100, "D")]
    //five times 1000 is out of roman numerals range (1 - 3999)
    fn roman_five_times_power_of_ten_is_specific_numeral(number: i32, numeral: &str) {
        assert_eq!(roman_rust::numeral_for(5*number), numeral);
    }

    #[test_case(1, "I", "V")]
    #[test_case(10, "X", "L")]
    #[test_case(100, "C", "D")]
    fn roman_six_times_power_of_ten_is_specific_numeral_for_five_times_and_once(number: i32, once: &str, five_times: &str) {
        assert_eq!(roman_rust::numeral_for(6*number), format!("{0}{1}", five_times, once));
    }

    #[test_case(1, "I", "V")]
    #[test_case(10, "X", "L")]
    #[test_case(100, "C", "D")]
    fn roman_seven_times_power_of_ten_is_specific_numeral_for_five_times_and_twice(number: i32, once: &str, five_times: &str) {
        assert_eq!(roman_rust::numeral_for(7*number), format!("{0}{1}{1}", five_times, once));
    }

    #[test_case(1, "I", "V")]
    #[test_case(10, "X", "L")]
    #[test_case(100, "C", "D")]
    fn roman_eight_times_power_of_ten_is_specific_numeral_for_five_times_and_thrice(number: i32, once: &str, five_times: &str) {
        assert_eq!(roman_rust::numeral_for(8*number), format!("{0}{1}{1}{1}", five_times, once));
    }

    #[test_case(1, "I", "V")]
    #[test_case(10, "X", "L")]
    #[test_case(100, "C", "D")]
    fn roman_four_times_power_of_ten_is_specific_numeral_for_once_and_five(number: i32, once: &str, five_times: &str) {
        assert_eq!(roman_rust::numeral_for(4*number), format!("{0}{1}", once, five_times));
    }

    #[test_case(1, "I", "X")]
    #[test_case(10, "X", "C")]
    #[test_case(100, "C", "M")]
    fn roman_nine_times_power_of_ten_is_specific_numeral_for_once_and_next_power_of_ten(number: i32, once: &str, ten_times: &str) {
        assert_eq!(roman_rust::numeral_for(9*number), format!("{0}{1}", once, ten_times));
    }

    #[test_case(1111, "MCXI")]
    #[test_case(999, "CMXCIX")]
    #[test_case(3999, "MMMCMXCIX")]
    fn other_numbers_are_concatenated_versions_of_the_previous_cases(number: i32, numeral: &str) {
        assert_eq!(roman_rust::numeral_for(number), numeral);
    }
}