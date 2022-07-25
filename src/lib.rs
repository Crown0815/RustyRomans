pub fn numeral_for(number: i32) -> String{
    let mut result = String::default();

    if number == 10 {
        return String::from("X")
    }
    if number == 100 {
        return String::from("C")
    }
    if number == 1000 {
        return String::from("M")
    }

    if number / 10 > 0{
        for _ in 0..number / 10 {
            result.push('X');
        }
        return result;
    }

    for _ in 0..number {
        result.push('I');
    }

    return result;
}