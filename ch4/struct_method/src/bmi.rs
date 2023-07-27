use std::fmt;

#[derive(Default, Debug)]
pub struct Body {
    height: f64,
    weight: f64,
}

impl Body {
    pub fn new(height: f64, weight: f64) -> Body {
        Body { height, weight }
    }

    pub fn calc_bmi(&self) -> f64 {
        self.weight / (self.height / 100.0).powf(2.0)
    }

    pub fn calc_per(&self) -> f64 {
        self.calc_bmi() / 22.0 * 100.0
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(height: {}, weight: {})", self.height, self.weight)
    }
}