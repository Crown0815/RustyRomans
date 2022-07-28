macro_rules! reduce {
    ($divisor: expr, $character: expr, $number: ident, $result: ident) => {
        if $number / $divisor > 0 {
            for _ in 0..$number / $divisor {
                $result.push_str($character);
            }
            $number = $number - ($number / $divisor) * $divisor;
        }
    }
}

#[allow(unused_assignments)]
pub fn numeral_for(number: i32) -> String{
    let mut result = String::default();
    let mut rest = number;

    reduce!(1000,"M", rest, result);
    reduce!(900 ,"CM",rest, result);
    reduce!(500 ,"D", rest, result);
    reduce!(400 ,"CD",rest, result);
    reduce!(100 ,"C", rest, result);
    reduce!(90  ,"XC",rest, result);
    reduce!(50  ,"L", rest, result);
    reduce!(40  ,"XL",rest, result);
    reduce!(10  ,"X", rest, result);
    reduce!(9   ,"IX",rest, result);
    reduce!(5   ,"V", rest, result);
    reduce!(4   ,"IV",rest, result);
    reduce!(1   ,"I", rest, result);

    return result;
}