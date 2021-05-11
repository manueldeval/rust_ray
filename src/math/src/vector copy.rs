use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::num::{One, Sqrt, Zero};

#[derive(Debug)]
pub struct Vec3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Clone for Vec3d<T>
where
    T: Clone,
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
    T: Default,
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
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T> Display for Vec3d<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector({},{},{})", self.x, self.y, self.z)
    }
}

impl<T> Add for Vec3d<T>
where
    T: Add<Output = T>,
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
    &'b T: Add<Output = T> + 'b,
{
    type Output = Vec3d<T>;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x_ref() + other.x_ref(),
            y: self.y_ref() + other.y_ref(),
            z: self.z_ref() + other.z_ref(),
        }
    }
}

impl<T> Neg for Vec3d<T>
where
    T: Neg<Output = T>,
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

impl<'b, T> Neg for &'b Vec3d<T>
where
    &'b T: Neg<Output = T> + 'b,
{
    type Output = Vec3d<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x_ref(),
            y: -self.y_ref(),
            z: -self.z_ref(),
        }
    }
}

impl<T> Sub for Vec3d<T>
where
    T: Sub<Output = T>,
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
{
    type Output = Vec3d<T>;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x_ref() - other.x_ref(),
            y: self.y_ref() - other.y_ref(),
            z: self.z_ref() - other.z_ref(),
        }
    }
}

impl<T> Vec3d<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    pub fn x_ref<'a>(&'a self) -> &'a T {
        &self.x
    }
    pub fn y_ref<'a>(&'a self) -> &'a T {
        &self.y
    }
    pub fn z_ref<'a>(&'a self) -> &'a T {
        &self.z
    }
    pub fn map<D, F>(&self, f: F) -> Vec3d<D>
    where
        F: Fn(T) -> D,
        T: Clone,
    {
        Vec3d {
            x: f(self.x.clone()),
            y: f(self.y.clone()),
            z: f(self.z.clone()),
        }
    }
}

pub trait Dot {
    type Output;
    fn dot(self, other: Self) -> Self::Output;
}

impl<T> Dot for Vec3d<T>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    type Output = T;

    fn dot(self, other: Self) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<'b, T> Dot for &'b Vec3d<T>
where
    &'b T: Mul<Output = T>,
    T: Add<Output = T>,
{
    type Output = T;

    fn dot(self, other: Self) -> Self::Output {
        self.x_ref() * other.x_ref() + self.y_ref() * other.y_ref() + self.z_ref() * other.z_ref()
    }
}

impl<'a, T> Vec3d<T>
where
    &'a T: Mul<Output = T> + 'a,
    T: Add<Output = T> + Sqrt,
{
    pub fn mag(&'a self) -> T {
        (self).dot(self).sqrt()
    }
}

#[derive(Debug)]
pub enum Vect3dError {
    DivideByZero,
}

impl<'a, T> Vec3d<T>
where
    &'a T: Mul<Output = T> + 'a,
    T: Add<Output = T> + Sqrt + One + Zero + PartialEq + Div<Output = T> + Mul<Output = T> + Clone,
{
    pub fn norm(&'a self) -> Result<Vec3d<T>, Vect3dError> {
        let mag = self.mag();
        if mag == T::zero() {
            Err(Vect3dError::DivideByZero)
        } else {
            let div: T = T::one() / mag;
            Ok(Self {
                x: self.x.clone() * div.clone(),
                y: self.y.clone() * div.clone(),
                z: self.z.clone() * div.clone(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        let v1: Vec3d<usize> = Vec3d::new(1, 2, 3);
        let v2: Vec3d<usize> = Vec3d::new(1, 5, 6);
        let v3: Vec3d<usize> = Vec3d::new(4, 2, 3);
        let v4: Vec3d<usize> = Vec3d::new(1, 2, 5);
        let v5: Vec3d<usize> = Vec3d::new(1, 2, 3);

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
    fn test_add_ref() {
        let v1: Vec3d<i16> = Vec3d::new(1, 2, 3);
        let v2: Vec3d<i16> = Vec3d::new(4, 5, 6);
        assert_eq!(&v1 + &v2, Vec3d::new(5, 7, 9));
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
        assert_eq!(v1.dot(v2), 2 + 2 * 4 + 3 * 6);
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

        assert_eq!(n, Vec3d::new(0.32444283,0.48666424,0.8111071));
    }
}
