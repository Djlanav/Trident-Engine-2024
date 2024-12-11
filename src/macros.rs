use crate::engine_types::FloatingPoint;
use std::ops::MulAssign;

#[macro_export]
macro_rules! implement_vector {
    ($struct_name:ident, $($args:ident),*) => {
        impl<T> $struct_name<T>
        where
            T: Sized + MulAssign + FloatingPoint
        {
            pub fn new($($args: T),*) -> Self {
                Self {
                    $($args),*
                }
            }
        }
    };
}