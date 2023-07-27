mod bmi;
mod person;

use bmi::Body;
use person::Person;
use std::{process::exit, io::{stdout, Write}};


fn get_f64_input() -> f64{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Input Error");

    match input.trim().parse::<f64>() {
        Ok(v) => v,
        Err(_) => { println!("Input type error."); exit(-1); }
    }
}

fn main() {
    print!("Input height: "); 
    let _ = stdout().flush();
    let height = get_f64_input();

    print!("Input weight: "); 
    let _ = stdout().flush();
    let weight = get_f64_input();

    let body = Body::new(height, weight);
    println!("Body : {}", body);
    println!("BMI = {:.2}", body.calc_bmi());
    println!("fat rate = {:.2}", body.calc_per());

    let bae = Person::new("BAE MUN SUNG".to_string(), 24, height, weight);
    println!("{}", bae);
}