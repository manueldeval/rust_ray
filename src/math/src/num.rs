use core::f64;
use std::ops::{Add, Div, Mul, Sub};


pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> f32 {
        f32::sqrt(self)
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> f64 {
        f64::sqrt(self)
    }
}

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_zero_for {
    ($type:ty,$value:expr) => (
        impl Zero for $type {
            fn zero() -> Self {
                $value
            }
        }
    );
}

impl_zero_for!(f32,0.0);
impl_zero_for!(f64,0.0);
impl_zero_for!(i8,0);
impl_zero_for!(i16,0);
impl_zero_for!(i32,0);
impl_zero_for!(i64,0);
impl_zero_for!(isize,0);

pub trait Trigo {
    fn cos(self) -> Self;
    fn sin(self) -> Self;
    fn acos(self) -> Self;
    fn asin(self) -> Self;
    fn tan(self) -> Self;
    fn atan(self) -> Self;
}

macro_rules! impl_trigo_for {
    ($type:ty) => (
        impl Trigo for $type {
            fn cos(self) -> Self { <$type>::cos(self) }
            fn acos(self) -> Self { <$type>::acos(self) }
            fn sin(self) -> Self { <$type>::sin(self) }
            fn asin(self) -> Self { <$type>::asin(self) }
            fn tan(self) -> Self { <$type>::tan(self) }
            fn atan(self) -> Self { <$type>::atan(self) }
        }
    );
}

impl_trigo_for!(f32);
impl_trigo_for!(f64);


pub trait One {
    fn one() -> Self;
}

macro_rules! impl_one_for {
    ($type:ty,$value:expr) => (
        impl One for $type {
            fn one() -> Self {
                $value
            }
        }
    );
}

impl_one_for!(f32,1.0);
impl_one_for!(f64,1.0);
impl_one_for!(i8,1);
impl_one_for!(i16,1);
impl_one_for!(i32,1);
impl_one_for!(i64,1);
impl_one_for!(isize,1);

pub trait ArithmeticOps: 
    Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Clone + PartialEq + Default + Zero + One
    where Self: std::marker::Sized {
}

macro_rules! impl_arithmetic_ops_for {
    ($type:ty) => (
        impl ArithmeticOps for $type {}
    );
}

impl_arithmetic_ops_for!(f32);
impl_arithmetic_ops_for!(f64);
impl_arithmetic_ops_for!(i8);
impl_arithmetic_ops_for!(i16);
impl_arithmetic_ops_for!(i32);
impl_arithmetic_ops_for!(i64);
impl_arithmetic_ops_for!(isize);
