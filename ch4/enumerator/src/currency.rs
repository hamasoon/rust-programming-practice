pub enum Currency {
    Currency100(isize),
    Currency500(isize),
    Currency1000(isize),
    Currency5000(isize),
    Currency10000(isize),
    Currency50000(isize),
}

impl Currency {
    pub fn price(&self) -> Option<i32> {
        match *self {
            Currency::Currency100(v) => {
                if v >= 0 {Option::Some((v as i32) * 100)} else {Option::None}
            },
            Currency::Currency500(v) => {
                if v >= 0 {Option::Some((v as i32) * 500)} else {Option::None}
            },
            Currency::Currency1000(v) => {
                if v >= 0 {Option::Some((v as i32) * 1000)} else {Option::None}
            },
            Currency::Currency5000(v) => {
                if v >= 0 {Option::Some((v as i32) * 5000)} else {Option::None}
            },
            Currency::Currency10000(v) => {
                if v >= 0 {Option::Some((v as i32) * 10000)} else {Option::None}
            },
            Currency::Currency50000(v) => {
                if v >= 0 {Option::Some((v as i32) * 50000)} else {Option::None}
            },
            _ => Option::None
        }
    }
}