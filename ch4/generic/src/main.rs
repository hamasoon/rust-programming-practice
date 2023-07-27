use std::fmt::{self, Debug};
use std::str::FromStr;
use std::ops::{Add, Sub};

fn get_input <T: FromStr>() -> T {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Input error");

    match input.trim().parse::<T>() {
        Ok(v) => v,
        Err(_) => { println!("Input type error"); std::process::exit(-1); }
    }
}

fn add<T>(n1: T, n2: T) -> T 
    where T: Add<Output = T>
{
    n1 + n2
}

#[derive(Debug)]
struct Point<T> 
    where T: FromStr
{
    x: T,
    y: T,
}

impl<T: Debug> fmt::Display for Point<T> where T: FromStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, {:?}", self.x, self.y)
    }
}


fn main() {
    let p1 = Point::<i32>{
        x: get_input::<i32>(),
        y: get_input::<i32>()
    };

    let p2 = Point::<f64>{
        x: get_input::<f64>(),
        y: get_input::<f64>()
    };

    println!("p1: {}", p1);
    println!("p2: {}", p2);
}