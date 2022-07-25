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
}