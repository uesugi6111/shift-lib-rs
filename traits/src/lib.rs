pub use self::traits::*;

mod traits {
    use num_traits::Bounded;
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
                type S = $bounded_type;
                fn identity() -> <Self as Monoid>::S {
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
    #[macro_export]
    macro_rules! array {
        ($($x:ident)+ *) => {
            ($($x)+*)
        };
    }

    struct Min<T: Ord + Bounded>(T);
    struct Max<T: Ord + Bounded>(T);
    impl_monoid!(Min<T:Ord + Bounded + Copy>,a b => *a.min(b),T::max_value());
    impl_monoid!(Max<T:Ord + Bounded + Copy>,a b => *a.min(b),T::min_value());
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
    impl_semigroup_by_symbol!(S,i64,+);
    assert_eq!(S::operator(&9, &5), 14);
}
