pub use self::general::*;

mod general {
    pub trait Assign: Ord + Copy {
        fn min_assign(&mut self, x: Self) {
            *self = (*self).min(x);
        }
        fn max_assign(&mut self, x: Self) {
            *self = (*self).max(x);
        }
    }
    impl<T:Ord + Copy> Assign for T {}
}

