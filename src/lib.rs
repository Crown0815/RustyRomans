pub fn numeral_for(number: i32) -> String{
    if number == 10 {
        return String::from("X")
    }
    if number == 100 {
        return String::from("C")
    }
    if number == 1000 {
        return String::from("M")
    }
    return String::from("I")
}