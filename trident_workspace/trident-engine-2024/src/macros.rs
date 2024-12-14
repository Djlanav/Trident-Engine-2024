#[macro_export]
macro_rules! implement_vector {
    ($struct_name:ident, $( $assoc_type:ty, $args:ident ),*) => {
        impl<T> $struct_name<T>
        where
            T: Sized + MulAssign + Clone + Copy
        {
            pub fn new($($args: T),*) -> Self {
                Self {
                    $($args),*
                }
            }

            pub fn modify<F>(&mut self, mut closure: F)
            where
                F: FnMut($(&mut $assoc_type),*)
            {
                closure($(&mut self.$args),*);
            }
        }
    };
}