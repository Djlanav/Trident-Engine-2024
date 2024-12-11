use std::ops::{Mul, MulAssign};
use crate::{implement_vector, macros};

pub trait FloatingPoint {}
impl FloatingPoint for f32 {}
impl FloatingPoint for f64 {}

pub trait Vector {
    type Scalar;
    fn multiply(&mut self, value: Self::Scalar);
    fn modify<F>(&mut self, closure: F);
}


implement_vector!(Vector2, x, y);
pub struct Vector2<T>
where
    T: Sized + FloatingPoint + MulAssign
{
    pub x: T,
    pub y: T
}


implement_vector!(Vector3, x, y, z);
pub struct Vector3<T>
where
    T: Sized + FloatingPoint + MulAssign
{
    pub x: T,
    pub y: T,
    pub z: T
}


implement_vector!(Vector4, x, y, z, w);
pub struct Vector4<T>
where
    T: Sized + FloatingPoint + MulAssign
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}