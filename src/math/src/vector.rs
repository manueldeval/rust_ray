use std::{fmt::Display, ops::{Add, Div, Mul, Neg, Sub}};

use crate::{error::Vect3dError, num::{ArithmeticOps, Sqrt, Trigo}};

#[derive(Debug,Serialize,Deserialize)]
pub struct Vec3d<T>
where
    T: ArithmeticOps,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Clone for Vec3d<T>
where
    T: ArithmeticOps,
{
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}

impl<T> Default for Vec3d<T>
where
    T: ArithmeticOps,
{
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}

impl<T> PartialEq for Vec3d<T>
where
    T: ArithmeticOps,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T> Display for Vec3d<T>
where
    T: Display + ArithmeticOps,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector({},{},{})", self.x, self.y, self.z)
    }
}

impl<T> Add for Vec3d<T>
where
    T: ArithmeticOps,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'b, T> Add for &'b Vec3d<T>
where
    &'b T: Add<Output = T>,
    T: ArithmeticOps
{
    type Output = Vec3d<T>;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: &self.x + &other.x,
            y: &self.y + &other.y,
            z: &self.z + &other.z,
        }
    }
}

impl<T> Neg for Vec3d<T>
where
    T: ArithmeticOps + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Sub for Vec3d<T>
where
    T: ArithmeticOps,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'b, T> Sub for &'b Vec3d<T>
where
    &'b T: Sub<Output = T>,
    T: ArithmeticOps
{
    type Output = Vec3d<T>;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: &self.x - &other.x,
            y: &self.y - &other.y,
            z: &self.z - &other.z,
        }
    }
}



impl<T> Vec3d<T>
where
    T: ArithmeticOps,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    pub fn each_mul(&self, t: T) -> Vec3d<T> {
        Self {
            x: self.x.clone() * t.clone(),
            y: self.y.clone() * t.clone(),
            z: self.z.clone() * t.clone(),
        }
    }
    pub fn each_div(&self, t: T) -> Vec3d<T> {
        Self {
            x: self.x.clone() / t.clone(),
            y: self.y.clone() / t.clone(),
            z: self.z.clone() / t.clone(),
        }
    }
    pub fn each_add(&self, t: T) -> Vec3d<T> {
        Self {
            x: self.x.clone() + t.clone(),
            y: self.y.clone() + t.clone(),
            z: self.z.clone() + t.clone(),
        }
    }
    pub fn each_sub(&self, t: T) -> Vec3d<T> {
        Self {
            x: self.x.clone() - t.clone(),
            y: self.y.clone() - t.clone(),
            z: self.z.clone() - t.clone(),
        }
    }
    pub fn map<D, F>(&self, f: F) -> Vec3d<D>
    where
        F: Fn(T) -> D,
        D: ArithmeticOps,
    {
        Vec3d {
            x: f(self.x.clone()),
            y: f(self.y.clone()),
            z: f(self.z.clone()),
        }
    }

    pub fn dot(&self, other: &Self) -> T {
        let v1 = self.clone();
        let v2 = other.clone();
        let res = v1 * v2;
        res.x + res.y + res.z
    }
    pub fn cross(&self, other: Self) -> Self {
        let v1 = self.clone();
        let v2 = other.clone();

        let x = v1.y.clone() * v2.z.clone() - v1.z.clone() * v2.y.clone();
        let y = v1.z.clone() * v2.x.clone() - v1.x.clone() * v2.z.clone();
        let z = v1.x.clone() * v2.y.clone() - v1.y.clone() * v2.x.clone();
        Self { x, y, z }
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
            z: T::one(),
        }
    }
    pub fn y_axis() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
            z: T::zero(),
        }
    }

    pub fn x_axis() -> Self {
        Self {
            x: T::one(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn with_value(t: T) -> Self {
        Self {
            x: t.clone(),
            y: t.clone(),
            z: t.clone(),
        }
    }
}

impl <T> Vec3d<T>
where T: ArithmeticOps + Trigo + Sqrt {
    pub fn angle(&self,other: &Vec3d<T>) -> T{
        self.norm().unwrap().dot(&other.norm().unwrap()).acos()
    }
}

impl<'a, T> Vec3d<T>
where
    T: ArithmeticOps + Sqrt,
{
    pub fn mag(&self) -> T {
        let a = self.clone();
        let b = self.clone();
        a.dot(&b).sqrt()
    }
}

impl<'a, T> Vec3d<T>
where
    T: ArithmeticOps + Sqrt,
{
    pub fn norm(&self) -> Result<Vec3d<T>, Vect3dError> {
        let mag = self.mag();
        if mag == T::zero() {
            Err(Vect3dError::DivideByZero)
        } else {
            let div: T = T::one() / mag;
            let other = self.clone();
            Ok(Self {
                x: other.x * div.clone(),
                y: other.y * div.clone(),
                z: other.z * div.clone(),
            })
        }
    }
}

impl <T> From<T> for Vec3d<T> 
where 
    T: ArithmeticOps 
{
    fn from(t: T) -> Self {
        Self {
            x: t.clone(),
            y: t.clone(),
            z: t.clone(),
        }
    }
} 

impl<T> Mul for Vec3d<T> 
where 
    T: ArithmeticOps 
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x.clone() * rhs.x.clone(),
            y: self.y.clone() * rhs.y.clone(),
            z: self.z.clone() * rhs.z.clone(),
        }
    }
} 

impl<'b, T> Mul for &'b Vec3d<T>
where
    &'b T: Mul<Output = T>,
    T: ArithmeticOps
{
    type Output = Vec3d<T>;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            x: &self.x * &other.x,
            y: &self.y * &other.y,
            z: &self.z * &other.z,
        }
    }
}

impl<T> Div for Vec3d<T> 
where 
    T: ArithmeticOps 
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x.clone() / rhs.x.clone(),
            y: self.y.clone() / rhs.y.clone(),
            z: self.z.clone() / rhs.z.clone(),
        }
    }
} 

impl<'b, T> Div for &'b Vec3d<T>
where
    &'b T: Div<Output = T>,
    T: ArithmeticOps
{
    type Output = Vec3d<T>;

    fn div(self, other: Self) -> Self::Output {
        Self::Output {
            x: &self.x / &other.x,
            y: &self.y / &other.y,
            z: &self.z / &other.z,
        }
    }
}


pub type Vec3dF32 = Vec3d<f32>;
pub type Vec3dF64 = Vec3d<f64>;
pub type Vec3dFI8 = Vec3d<i8>;
pub type Vec3dFI16 = Vec3d<i16>;
pub type Vec3dFI32 = Vec3d<i32>;
pub type Vec3dFI64 = Vec3d<i64>;
pub type Vec3dIsize = Vec3d<isize>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        let v1: Vec3d<i32> = Vec3d::new(1, 2, 3);
        let v2: Vec3d<i32> = Vec3d::new(1, 5, 6);
        let v3: Vec3d<i32> = Vec3d::new(4, 2, 3);
        let v4: Vec3d<i32> = Vec3d::new(1, 2, 5);
        let v5: Vec3d<i32> = Vec3d::new(1, 2, 3);

        assert_eq!(v1, v5);
        assert_ne!(v1, v2);
        assert_ne!(v1, v3);
        assert_ne!(v1, v4);
    }

    #[test]
    fn test_default() {
        let v: Vec3d<i16> = Vec3d::default();
        assert_eq!(v, Vec3d::new(0, 0, 0));
    }

    #[test]
    fn test_clone() {
        let v: Vec3d<i16> = Vec3d::new(1, 2, 3);
        assert_eq!(v.clone(), Vec3d::new(1, 2, 3));
    }

    #[test]
    fn test_add() {
        let v1: Vec3d<i16> = Vec3d::new(1, 2, 3);
        let v2: Vec3d<i16> = Vec3d::new(4, 5, 6);
        assert_eq!(v1 + v2, Vec3d::new(5, 7, 9));
    }

    #[test]
    fn test_sub() {
        let v1: Vec3d<i16> = Vec3d::new(1, 2, 3);
        let v2: Vec3d<i16> = Vec3d::new(4, 1, 7);
        assert_eq!(v1 - v2, Vec3d::new(-3, 1, -4));
    }

    #[test]
    fn test_neg() {
        let v1: Vec3d<i16> = Vec3d::new(1, 2, 3);
        assert_eq!(-v1, Vec3d::new(-1, -2, -3));
    }

    #[test]
    fn test_dot() {
        let v1: Vec3d<i16> = Vec3d::new(1, 2, 3);
        let v2: Vec3d<i16> = Vec3d::new(2, 4, 6);
        assert_eq!(v1.dot(&v2), 2 + 2 * 4 + 3 * 6);
    }

    #[test]
    fn test_mag() {
        let v1: Vec3d<f64> = Vec3d::new(2.0, 3.0, 5.0);
        assert!(approx_equal(v1.mag(), 6.164414002968976, 10));
    }

    fn approx_equal(a: f64, b: f64, decimal_places: u8) -> bool {
        let factor = 10.0f64.powi(decimal_places as i32);
        let a = (a * factor).trunc();
        let b = (b * factor).trunc();
        a == b
    }

    #[test]
    fn test_into() {
        let v1: Vec3d<i16> = Vec3d::new(2, 3, 5);
        let v2 = v1.map(|v| v as f32);
        assert_eq!(v2, Vec3d::new(2.0, 3.0, 5.0));
    }

    #[test]
    fn test_norm() {
        let v1: Vec3d<f32> = Vec3d::new(2.0, 3.0, 5.0);
        let n = v1.norm().unwrap();

        assert_eq!(n, Vec3d::new(0.32444283, 0.48666424, 0.8111071));
    }

    #[test]
    fn test_cross() {
        let v1: Vec3d<i32> = Vec3d::new(1, 2, 3);
        let v2: Vec3d<i32> = Vec3d::new(4, 5, 6);

        assert_eq!(v1.cross(v2), Vec3d::new(-3, 6, -3));
    }


    #[test]
    fn test_angle() {
        let v1: Vec3d<f32> = Vec3d::new(0.0, 0.0, 1.0);
        let v2: Vec3d<f32> = Vec3d::new(0.0, 10.0, 0.0);
        assert_eq!(v1.angle(&v2),1.5707964 );
    }
}
