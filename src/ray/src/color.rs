use std::{iter::Sum, ops::{Add, Mul}};

pub const WHITE: Color = Color{r: 1.0, g: 1.0, b:1.0};
pub const BLACK: Color = Color{r: 0.0, g: 0.0, b:0.0};
#[derive(Clone,Serialize,Deserialize)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn scale(&self, k: f64) -> Color {
        Self::new(k * self.r, k * self.g, k * self.b)
    }

    fn set_max_to_one(value: f64) -> f64 {
        if value > 1.0 {
            1.0
        } else {
            value
        }
    }

    pub fn to_drawing_color(&self) -> Color {
        Self {
            r: 255.0 * Color::set_max_to_one(self.r),
            g: 255.0 * Color::set_max_to_one(self.g),
            b: 255.0 * Color::set_max_to_one(self.b),
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}


impl Add for &Color {
    type Output = Color;
    fn add(self, other: Self) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul for &Color {
    type Output = Color;
    fn mul(self, other: Self) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Sum for Color {

    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color { r: 0.0, g: 0.0, b: 0.0 }, |a, b| Color {
            r: a.r + b.r,
            g: a.g + b.g,
            b: a.b + b.b,
        })
    }
}