#[cfg(test)]
mod tests {

    #[test]
    fn roman_one_is_i() {
        assert_eq!(roman_rust::numeral_for(1), "I");
    }

    #[test]
    fn roman_ten_is_x() {
        assert_eq!(roman_rust::numeral_for(10), "X");
    }

    #[test]
    fn roman_hundred_is_c() {
        assert_eq!(roman_rust::numeral_for(100), "C");
    }

    #[test]
    fn roman_thousand_is_m() {
        assert_eq!(roman_rust::numeral_for(1000), "M");
    }

    #[test]
    fn roman_two_is_twice_i() {
        assert_eq!(roman_rust::numeral_for(2), "II");
    }

    #[test]
    fn roman_three_is_thrice_i() {
        assert_eq!(roman_rust::numeral_for(3), "III");
    }

    #[test]
    fn roman_twenty_is_twice_x() {
        assert_eq!(roman_rust::numeral_for(20), "XX");
    }


}