macro_rules! reduce {
    ($number: expr, $divisor: expr, $character: expr, $result: ident) => {
        if $number / $divisor > 0 {
            for _ in 0..$number / $divisor {
                $result.push($character);
            }
            return $result;
        }
    }
}

pub fn numeral_for(number: i32) -> String{
    let mut result = String::default();

    reduce!(number, 1000,'M', result);
    reduce!(number, 100,'C', result);
    reduce!(number, 10,'X', result);
    reduce!(number, 1,'I', result);

    return String::default();
}