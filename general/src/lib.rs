pub use self::general::*;

mod general {
    #[macro_export]
    macro_rules! max_assign {
        ($left:expr,$right:expr) => {
            if $left < $right {
                $left = $right;
            }
        };
    }
    #[macro_export]
    macro_rules! min_assign {
        ($left:expr,$right:expr) => {
            if $left > $right {
                $left = $right;
            }
        };
    }
    pub fn ceil_pow2(n: u32) -> u32 {
        32 - n.saturating_sub(1).leading_zeros()
    }
    pub fn msb(n: u32) -> u32 {
        31 - n.leading_zeros()
    }
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    pub enum OrdOption<T: Ord> {
        Min,
        Val(T),
        Max,
    }
    impl<T: Ord + num_traits::CheckedAdd + num_traits::Zero> std::ops::Add for OrdOption<T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Self::Min, OrdOption::Max) | (Self::Max, Self::Min) => {
                    panic!("inf - inf is undefined")
                }
                (Self::Max, _) | (_, Self::Max) => Self::Max,
                (Self::Min, _) | (_, Self::Min) => Self::Min,
                (Self::Val(x), Self::Val(y)) => match x.checked_add(&y) {
                    Some(ans) => Self::Val(ans),
                    _ => {
                        if x > T::zero() {
                            Self::Max
                        } else {
                            Self::Min
                        }
                    }
                },
            }
        }
    }
    impl<T: Ord + std::ops::Mul<Output = T> + num_traits::Zero> std::ops::Mul for OrdOption<T> {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            todo!()
            /*
            match (self, rhs) {
                (Self::Max,Self::Max) => Self::Max,
                (Self::Max,x) | (x,Self::Max) => {if x < Self::Val(T::zero()) {Self::Min} else {Self::Max}},
                (Self::)
                _ => todo!()
            }
            */
        }
    }
    impl<T: Ord + std::fmt::Display> std::fmt::Display for OrdOption<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                OrdOption::Min | OrdOption::Max => write!(f, "-1"),
                OrdOption::Val(x) => write!(f, "{}", x),
            }
        }
    }
}

#[test]
fn t() {
    type OO<T> = OrdOption<T>;
    let v = OO::Val(10);
    assert!(v < OO::Max);
    assert!(OO::Min < v);
    assert!(v + OO::Max == OO::Max);
}
