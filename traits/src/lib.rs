pub use self::traits::*;

mod traits {
    pub trait SemiGroup {
        type S: Clone;
        fn operator(a: &Self::S, b: &Self::S) -> Self::S;
    }
    pub trait ComSemiGroup: SemiGroup {
        type S: Clone;
    }
    pub trait Monoid: SemiGroup {
        type S: Clone;
        fn identity() -> <Self as Monoid>::S;
    }
    pub trait ComMonoid: Monoid {
        type S: Clone;
    }
    pub trait Group: Monoid {
        type S: Clone;
        fn inverse(a: &<Self as Group>::S) -> <Self as Group>::S;
    }
    pub trait ComGroup: Group {
        type S: Clone;
    }
    #[macro_export]
    macro_rules! impl_semigroup {
        ($wr:ty,$t:ty,$op:expr) => {
            impl SemiGroup for $wr {
                type S = $t;
                fn operator(a: &Self::S, b: &Self::S) -> Self::S {
                    $op(a, b)
                }
            }
        };
        ($wr:ty,$t:ty,$a:tt $b:tt => $res:expr) => {
            impl SemiGroup for $wr {
                type S = $t;
                fn operator($a: &Self::S, $b: &Self::S) -> Self::S {
                    $res
                }
            }
        };
        ($t:ty,$op:expr) => {
            impl SemiGroup for $t {
                type S = $t;
                fn operator(a: &Self::S, b: &Self::S) -> Self::S {
                    $op(a, b)
                }
            }
        };
        ($t:ty,$a:tt $b:tt => $res:expr) => {
            impl SemiGroup for $t {
                type S = $t;
                fn operator($a: &Self::S, $b: &Self::S) -> Self::S {
                    $res
                }
            }
        };
    }
    #[macro_export]
    macro_rules! impl_semigroup_by_symbol {
        ($wr:ty,$t:ty,$op_symbol:tt) => {
            impl SemiGroup for $wr {
                type S = $t;
                fn operator(a:&Self::S,b:&Self::S) -> Self::S {
                    a $op_symbol b
                }
            }
        };
        ($t:ty,$op_symbol:tt) => {
            impl SemiGroup for $t {
                type S = $t;
                fn operator(a:&Self::S,b:&Self::S) -> Self::S {
                    a $op_symbol b
                }
            }
        }
    }
    #[macro_export]
    macro_rules! impl_monoid {
        ($wr:ty,$t:ty,$op:expr,$id:expr) => {
            impl_semigroup!($wr,$t, $op);
            impl Monoid for $wr {
                type S = $t;
                fn identity() -> <Self as traits::Monoid>::S {
                    $id
                }
            }
        };
        ($t:ty,$op:expr,$id:expr) => {
            impl_semigroup!($t, $op);
            impl Monoid for $t {
                type S = $t;
                fn identity() -> <Self as traits::Monoid>::S {
                    $id
                }
            }
        };
        ($wr:ty,$t:ty,$a:tt $b:tt => $ans:expr,$id:expr) => {
            impl_semigroup!($wr,$t,$a $b => $ans);
            impl Monoid for $wr {
                type S = $t;
                fn identity() -> <Self as traits::Monoid>::S {
                    $id
                }
            }
        };
        ($t:ty,$a:tt $b:tt => $ans:expr,$id:expr) => {
            impl_semigroup!($t,$a $b => $ans);
            impl Monoid for $t {
                type S = $t;
                fn identity() -> <Self as traits::Monoid>::S {
                    $id
                }
            }
        }
    }
    #[macro_export]
    macro_rules! impl_group {
        ($wr:ty,$t:ty,$op:expr,$id:expr,$inv:expr) => {
            impl_monoid!($wr,$t, $op, $id);
            impl Group for $wr {
                type S = $t;
                fn inverse(a: &<Self as Group>::S) -> <Self as Group>::S {
                    $inv(a)
                }
            }
        };
        ($wr:ty,$t:ty,$a:tt $b:tt => $ans:expr,$id:expr,$c:tt => $d:expr) => {
            impl_monoid!($wr,$t,$a $b => $ans,$id);
            impl Group for $wr {
                type S = $t;
                fn inverse($c: &<Self as Group>::S) -> <Self as Group>::S {
                    $d
                }
            }
        };
        ($t:ty,$op:expr,$id:expr,$inv:expr) => {
            impl_monoid!($t, $op, $id);
            impl Group for $t {
                type S = $t;
                fn inverse(a: &<Self as Group>::S) -> <Self as Group>::S {
                    $inv(a)
                }
            }
        };
        ($t:ty,$a:tt $b:tt => $ans:expr,$id:expr,$c:tt => $d:expr) => {
            impl_monoid!($t,$a $b => $ans,$id);
            impl Group for $t {
                type S = $t;
                fn inverse($c: &<Self as Group>::S) -> <Self as Group>::S {
                    $d
                }
            }
        }
    }
}

#[test]
fn t() {
    impl_group!(i32,a b => a+b,0,a => -a);
    assert_eq!(i32::operator(&1, &2), 3);
    assert_eq!(i32::identity(), 0);
    assert_eq!(i32::inverse(&1), -1);
}

#[test]
fn tuple_group() {
    impl_semigroup!((i64,i64),(a,b) (c,d) => (a+c,b+d));
    assert_eq!(<(i64, i64)>::operator(&(1, 2), &(3, 4)), (4, 6));
}

#[test]
fn impl_by_symbol() {
    impl_semigroup_by_symbol!(i64,+);
    assert_eq!(i64::operator(&9, &5), 14);
}
