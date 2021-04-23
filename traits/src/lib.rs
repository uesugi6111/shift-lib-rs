pub use self::traits::*;

mod traits {

    use num_traits::{Bounded, One, Zero};
    pub trait SemiGroup {
        type S: Clone;
        fn operator(a: &Self::S, b: &Self::S) -> Self::S;
    }
    pub trait ComSemiGroup: SemiGroup {
    }
    pub trait Monoid: SemiGroup {
        fn identity() -> Self::S;
        fn pow(a:&Self::S,mut n:usize) -> Self::S {
            let mut ret = Self::identity();
            let mut mul = a.clone();
            while n > 0 {
                if n % 2 == 1 {
                    ret = Self::operator(&ret, &mul).into();
                }
                mul = Self::operator(&mul, &mul);
                n /= 2;
            }
            ret
        }
    }
    pub trait ComMonoid: Monoid {
    }
    pub trait Group: Monoid {
        fn inverse(a: &Self::S) -> Self::S;
    }
    pub trait ComGroup: Group {
    }
    #[macro_export]
    macro_rules! impl_semigroup {
        ($wr:ident,$t:ty,$op:expr) => {
            #[derive(Clone)]
            struct $wr($t);
            impl SemiGroup for $wr {
                type S = $t;
                fn operator(a: &Self::S, b: &Self::S) -> Self::S {
                    $op(a, b)
                }
            }
        };
        ($wr:ident,$t:ty,$a:tt $b:tt => $res:expr) => {
            #[derive(Clone)]
            struct $wr($t);
            impl SemiGroup for $wr {
                type S = $t;
                fn operator($a: &Self::S, $b: &Self::S) -> Self::S {
                    $res
                }
            }
        };
        ($g:ident<$bounded_type:ident: $bound:tt $(+ $others:tt )*>,$a:tt $b:tt => $res:expr) => {
            impl<$bounded_type> SemiGroup for $g<$bounded_type>
            where $bounded_type : $bound $(+ $others)*
            {
                type S = T;
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
        ($wr:ident,$t:ty,$op_symbol:tt) => {
            #[derive(Clone)]
            struct $wr($t);
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
        ($wr:ident,$t:ty,$op:expr,$id:expr) => {
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
        ($wr:ident,$t:ty,$a:tt $b:tt => $ans:expr,$id:expr) => {
            impl_semigroup!($wr,$t,$a $b => $ans);
            impl Monoid for $wr {
                type S = $t;
                fn identity() -> <Self as traits::Monoid>::S {
                    $id
                }
            }
        };
        ($g:ident<$bounded_type:ident: $bound:tt $(+ $others:tt )*>,$a:tt $b:tt => $res:expr,$id:expr) => {
            impl_semigroup!($g<$bounded_type: $bound $(+ $others)*>,$a $b => $res);
            impl<$bounded_type> Monoid for $g<$bounded_type>
            where $bounded_type : $bound $(+ $others)*
            {
                fn identity() -> <Self>::S {
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
        };

    }
    #[macro_export]
    macro_rules! impl_group {
        ($wr:ident,$t:ty,$op:expr,$id:expr,$inv:expr) => {
            impl_monoid!($wr,$t, $op, $id);
            impl Group for $wr {
                type S = $t;
                fn inverse(a: &<Self as Group>::S) -> <Self as Group>::S {
                    $inv(a)
                }
            }
        };
        ($wr:ident,$t:ty,$a:tt $b:tt => $ans:expr,$id:expr,$c:tt => $d:expr) => {
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
        };
        ($g:ident<$bounded_type:ident: $bound:tt $(+ $others:tt )*>,$a:tt $b:tt => $res:expr,$id:expr,$c:tt => $d:expr) => {
            impl_monoid!($g:ident<$bounded_type:ident: $bound:tt $(+ $others:tt )*>,$a:tt $b:tt => $res:expr,$id);
            impl<$bounded_type> Group for $g<$bounded_type>
            where $bounded_type : $bound $(+ $others)*
            {
                type S = $bounded_type;
                fn inverse($c: &<Self as Group>::S) -> <Self as Group>::S {
                    $d
                }
            }
        };
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
    pub struct Min<T: Ord + Bounded + Clone>(T);
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
    pub struct Max<T:  Bounded + Clone>(T);
    impl_monoid!(Min<T:Ord + Bounded + Clone>,a b => a.clone().max(b.clone()),T::max_value());
    impl_monoid!(Max<T:Ord + Bounded + Clone>,a b => a.clone().min(b.clone()),T::min_value());
    #[derive(Clone)]
    pub struct Additive<T: Clone + Zero>(T);
    #[derive(Clone)]
    pub struct Multiplicative<T: Clone + One>(T);
    impl_monoid!(Additive<T: Clone + Zero>,a b => a.clone().add(b.clone()),T::zero());
    impl_monoid!(Multiplicative<T: Clone + One>,a b => a.clone().mul(b.clone()),T::one());

}



#[test]
fn impl_by_symbol() {
    impl_semigroup_by_symbol!(S,i64,+);
    assert_eq!(S::operator(&9, &5), 14);
}
#[test]
fn add_mul() {
    type T = Additive<i64>;
    assert_eq!(T::operator(&1, &-200),-199);
}