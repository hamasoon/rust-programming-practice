mod currency;

use std::io::Write;
use currency::Currency;

fn get_input() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Input error");

    match input.trim().parse::<i32>() {
        Ok(v) => v,
        Err(_) => { print!("Input type error"); std::process::exit(-1); }
    }
}

fn concurrent_input() -> Vec<Currency> {
    let mut ret: Vec<Currency> = Vec::new();

    print!("100 Input: "); let _ = std::io::stdout().flush();
    ret.push(Currency::Currency100(get_input() as isize));
    print!("500 Input: "); let _ = std::io::stdout().flush();
    ret.push(Currency::Currency500(get_input() as isize));
    print!("1000 Input: "); let _ = std::io::stdout().flush();
    ret.push(Currency::Currency1000(get_input() as isize));
    print!("5000 Input: "); let _ = std::io::stdout().flush();
    ret.push(Currency::Currency5000(get_input() as isize));
    print!("10000 Input: "); let _ = std::io::stdout().flush();
    ret.push(Currency::Currency10000(get_input() as isize));
    print!("50000 Input: "); let _ = std::io::stdout().flush();
    ret.push(Currency::Currency50000(get_input() as isize));

    ret
}

fn main() {
    println!("Hello, world!");
    let currency_vec = concurrent_input();
    let mut ret = 0;

    for c in currency_vec.iter() {
        ret += match c.price() {
            Option::Some(v) => v,
            Option::None => {
                print!("Wrong count");
                std::process::exit(-2);
            }
        }
    }

    print!("{}", ret);

    print!("{}", match get_input() {
        ..=-1 => "Unborn",
        0..=6 => "Baby",
        7..=12 => "Elemetary",
        13..=15 => "Middle",
        16..=18 => "High",
        19 => "Flesh Man",
        20 => "Sophomore",
        21 => "Junior",
        22 => "Senior",
        23..=59 => "Adult",
        60.. => "Old man"
    });
}
