use crate::implement_vector;
use std::ops::MulAssign;

pub struct Vector2<T>
where
    T: Sized + MulAssign + Clone + Copy
{
    pub x: T,
    pub y: T
}

pub struct Vector3<T>
where
    T: Sized + MulAssign + Clone + Copy
{
    pub x: T,
    pub y: T,
    pub z: T
}

pub struct Vector4<T>
where
    T: Sized + MulAssign + Clone + Copy
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

implement_vector!(Vector2, T, x, T, y);
implement_vector!(Vector3, T, x, T, y, T, z);
implement_vector!(Vector4, T, x, T, y, T, z, T, w);