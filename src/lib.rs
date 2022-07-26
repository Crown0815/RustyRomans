macro_rules! reduce {
    ($number: ident, $divisor: expr, $character: expr, $result: ident) => {
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

    reduce!(rest, 1000,"M", result);
    reduce!(rest, 900,"CM", result);
    reduce!(rest, 500,"D", result);
    reduce!(rest, 400,"CD", result);
    reduce!(rest, 100,"C", result);
    reduce!(rest, 90,"XC", result);
    reduce!(rest, 50,"L", result);
    reduce!(rest, 40,"XL", result);
    reduce!(rest, 10,"X", result);
    reduce!(rest, 9,"IX", result);
    reduce!(rest, 5,"V", result);
    reduce!(rest, 4,"IV", result);
    reduce!(rest, 1,"I", result);

    return result;
}