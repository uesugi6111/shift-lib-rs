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
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
    impl<T: Ord + num_traits::CheckedMul + num_traits::Zero> std::ops::Mul for OrdOption<T> {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            if self == Self::Val(T::zero()) || rhs == Self::Val(T::zero()) {
                Self::Val(T::zero())
            } else {
                match (self, rhs) {
                    (Self::Max, Self::Max) => Self::Max,
                    (Self::Max, x) | (x, Self::Max) => {
                        if x < Self::Val(T::zero()) {
                            Self::Min
                        } else {
                            Self::Max
                        }
                    }
                    (Self::Val(x), Self::Val(y)) => match x.checked_mul(&y) {
                        Some(ans) => Self::Val(ans),
                        _ => {
                            if (x > T::zero() && y > T::zero()) || (x < T::zero() && y < T::zero())
                            {
                                Self::Max
                            } else {
                                Self::Min
                            }
                        }
                    },
                    _ => todo!(),
                }
            }
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
    pub fn print_oneline<I: IntoIterator>(a: I)
    where
        I::Item: std::fmt::Display,
    {
        let mut a = a.into_iter();
        if let Some(v) = a.next() {
            print!("{}", v);
            for v in a {
                print!(" {}", v)
            }
            println!();
        }
    }
    pub fn print_lines<I: IntoIterator>(a: I)
    where
        I::Item: std::fmt::Display,
    {
        let a = a.into_iter();
        for v in a {
            println!("{}", v);
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

#[test]
fn test_output() {
    let v = [5, 2, 1, 4, 9];
    let vv = vec![5, 2, 1, 4, 9];
    let null:Vec<i32> = Vec::new();
    print_lines(&null);
    print_oneline(&null);
    print_lines(&vv);
    print_oneline(&vv);
}

