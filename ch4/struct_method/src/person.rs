use super::bmi::Body;
use std::fmt;

#[derive(Default, Debug)]
pub struct Person {
    name: String,
    age: u32,
    body: Body
}

impl Person {
    pub fn new(name: String, age: u32, height: f64, weight: f64) -> Person {
        Person { name, age, body: Body::new(height, weight) }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}, age: {}, {}", self.name, self.age, self.body)
    }
}