#[derive(Clone)]
struct Cache {
    rest: i32,
    roman: String,
}

impl Cache {
    fn reduce(&self, value: i32, letter: &str) -> Cache {
        let mut temp = self.clone();
        while temp.rest >= value {
            temp = temp.transfer(value, letter);
        }
        return temp;
    }

    fn transfer(&self, number: i32, letter: &str) -> Cache {
        Cache {
            rest: self.rest - number,
            roman: format!("{}{}", self.roman, letter),
        }
    }
}

pub fn numeral_for(number: i32) -> String {
    let tmp = Cache {
        rest: number,
        roman: String::default(),
    };

    return tmp.reduce(1000, "M")
        .reduce(900, "CM")
        .reduce(500, "D")
        .reduce(400, "CD")
        .reduce(100, "C")
        .reduce(90, "XC")
        .reduce(50, "L")
        .reduce(40, "XL")
        .reduce(10, "X")
        .reduce(9, "IX")
        .reduce(5, "V")
        .reduce(4, "IV")
        .reduce(1, "I")
        .roman;
}
