use std::{fmt::Display, ops::{Add, Div, Mul, Neg, Sub}};

use crate::{error::Vect3dError, num::{ArithmeticOps, Sqrt, Trigo}};

#[derive(Debug,Serialize,Deserialize)]
pub struct Vec2d<T>
where
    T: ArithmeticOps,
{
    pub x: T,
    pub y: T,
}

impl<T> Clone for Vec2d<T>
where
    T: ArithmeticOps,
{
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T> Default for Vec2d<T>
where
    T: ArithmeticOps,
{
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T> PartialEq for Vec2d<T>
where
    T: ArithmeticOps,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Display for Vec2d<T>
where
    T: Display + ArithmeticOps,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector({},{})", self.x, self.y)
    }
}

impl<T> Add for Vec2d<T>
where
    T: ArithmeticOps,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'b, T> Add for &'b Vec2d<T>
where
    &'b T: Add<Output = T>,
    T: ArithmeticOps
{
    type Output = Vec2d<T>;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: &self.x + &other.x,
            y: &self.y + &other.y,
        }
    }
}

impl<T> Neg for Vec2d<T>
where
    T: ArithmeticOps + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> Sub for Vec2d<T>
where
    T: ArithmeticOps,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'b, T> Sub for &'b Vec2d<T>
where
    &'b T: Sub<Output = T>,
    T: ArithmeticOps
{
    type Output = Vec2d<T>;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: &self.x - &other.x,
            y: &self.y - &other.y,
        }
    }
}



impl<T> Vec2d<T>
where
    T: ArithmeticOps,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn each_mul(&self, t: T) -> Vec2d<T> {
        Self {
            x: self.x.clone() * t.clone(),
            y: self.y.clone() * t.clone(),
        }
    }
    pub fn each_div(&self, t: T) -> Vec2d<T> {
        Self {
            x: self.x.clone() / t.clone(),
            y: self.y.clone() / t.clone(),
        }
    }
    pub fn each_add(&self, t: T) -> Vec2d<T> {
        Self {
            x: self.x.clone() + t.clone(),
            y: self.y.clone() + t.clone(),
        }
    }
    pub fn each_sub(&self, t: T) -> Vec2d<T> {
        Self {
            x: self.x.clone() - t.clone(),
            y: self.y.clone() - t.clone(),
        }
    }
    pub fn map<D, F>(&self, f: F) -> Vec2d<D>
    where
        F: Fn(T) -> D,
        D: ArithmeticOps,
    {
        Vec2d {
            x: f(self.x.clone()),
            y: f(self.y.clone()),
        }
    }

    pub fn dot(&self, other: &Self) -> T {
        let v1 = self.clone();
        let v2 = other.clone();
        let res = v1 * v2;
        res.x + res.y
    }


    //      z
    //      |
    //      |_____ y
    //     /
    //    / x
    pub fn z_axis() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
    pub fn y_axis() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
        }
    }

    pub fn x_axis() -> Self {
        Self {
            x: T::one(),
            y: T::zero(),
        }
    }

    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn with_value(t: T) -> Self {
        Self {
            x: t.clone(),
            y: t.clone(),
        }
    }
}

impl <T> Vec2d<T>
where T: ArithmeticOps + Trigo + Sqrt {
    pub fn angle(&self,other: &Vec2d<T>) -> T{
        self.norm().unwrap().dot(&other.norm().unwrap()).acos()
    }
}

impl<'a, T> Vec2d<T>
where
    T: ArithmeticOps + Sqrt,
{
    pub fn mag(&self) -> T {
        let a = self.clone();
        let b = self.clone();
        a.dot(&b).sqrt()
    }
}

impl<'a, T> Vec2d<T>
where
    T: ArithmeticOps + Sqrt,
{
    pub fn norm(&self) -> Result<Vec2d<T>, Vect3dError> {
        let mag = self.mag();
        if mag == T::zero() {
            Err(Vect3dError::DivideByZero)
        } else {
            let div: T = T::one() / mag;
            let other = self.clone();
            Ok(Self {
                x: other.x * div.clone(),
                y: other.y * div.clone(),
            })
        }
    }
}

impl <T> From<T> for Vec2d<T> 
where 
    T: ArithmeticOps 
{
    fn from(t: T) -> Self {
        Self {
            x: t.clone(),
            y: t.clone(),
        }
    }
} 

impl<T> Mul for Vec2d<T> 
where 
    T: ArithmeticOps 
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x.clone() * rhs.x.clone(),
            y: self.y.clone() * rhs.y.clone(),
        }
    }
} 

impl<'b, T> Mul for &'b Vec2d<T>
where
    &'b T: Mul<Output = T>,
    T: ArithmeticOps
{
    type Output = Vec2d<T>;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            x: &self.x * &other.x,
            y: &self.y * &other.y,
        }
    }
}

impl<T> Div for Vec2d<T> 
where 
    T: ArithmeticOps 
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x.clone() / rhs.x.clone(),
            y: self.y.clone() / rhs.y.clone(),
        }
    }
} 

impl<'b, T> Div for &'b Vec2d<T>
where
    &'b T: Div<Output = T>,
    T: ArithmeticOps
{
    type Output = Vec2d<T>;

    fn div(self, other: Self) -> Self::Output {
        Self::Output {
            x: &self.x / &other.x,
            y: &self.y / &other.y,
        }
    }
}


pub type Vec2dF32 = Vec2d<f32>;
pub type Vec2dF64 = Vec2d<f64>;
pub type Vec2dFI8 = Vec2d<i8>;
pub type Vec2dFI16 = Vec2d<i16>;
pub type Vec2dFI32 = Vec2d<i32>;
pub type Vec2dFI64 = Vec2d<i64>;
pub type Vec2dIsize = Vec2d<isize>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        let v1: Vec2d<i32> = Vec2d::new(1, 2);
        let v5: Vec2d<i32> = Vec2d::new(1, 2);

        assert_eq!(v1, v5);
    }

    #[test]
    fn test_default() {
        let v: Vec2d<i16> = Vec2d::default();
        assert_eq!(v, Vec2d::new(0, 0));
    }

    #[test]
    fn test_clone() {
        let v: Vec2d<i16> = Vec2d::new(1, 2);
        assert_eq!(v.clone(), Vec2d::new(1, 2));
    }

    #[test]
    fn test_add() {
        let v1: Vec2d<i16> = Vec2d::new(1, 2);
        let v2: Vec2d<i16> = Vec2d::new(4, 5);
        assert_eq!(v1 + v2, Vec2d::new(5, 7));
    }

    #[test]
    fn test_sub() {
        let v1: Vec2d<i16> = Vec2d::new(1, 2);
        let v2: Vec2d<i16> = Vec2d::new(4, 1);
        assert_eq!(v1 - v2, Vec2d::new(-3, 1));
    }

    #[test]
    fn test_neg() {
        let v1: Vec2d<i16> = Vec2d::new(1, 2);
        assert_eq!(-v1, Vec2d::new(-1, -2));
    }

    #[test]
    fn test_dot() {
        let v1: Vec2d<i16> = Vec2d::new(1, 2);
        let v2: Vec2d<i16> = Vec2d::new(2, 4);
        assert_eq!(v1.dot(&v2), 2 + 2 * 4 );
    }

}
