pub use self::algebraic_structures_impl::*;
mod algebraic_structures_impl {
    use std::ops::Neg;

    use __traits::*;
    use num_traits::{Bounded, One, Zero};
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
    pub struct Min<T: Ord + Clone>(T);
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
    pub struct Max<T: Ord + Clone>(T);
    impl_semigroup!(Min<T:Ord + Clone>,a b => a.clone().min(b.clone()));
    impl_semigroup!(Max<T:Ord + Clone>,a b => a.clone().max(b.clone()));
    impl<T: Ord + Clone + Bounded> Monoid for Min<T> {
        fn identity() -> Self::S {
            T::max_value()
        }
    }
    impl<T: Ord + Clone + Bounded> Monoid for Max<T> {
        fn identity() -> Self::S {
            T::min_value()
        }
    }

    #[derive(Clone)]
    pub struct Additive<T: Clone + Zero>(T);
    #[derive(Clone)]
    pub struct Multiplicative<T: Clone + One>(T);
    impl_monoid!(Additive<T: Clone + Zero>,a b => a.clone().add(b.clone()),T::zero());
    impl_monoid!(Multiplicative<T: Clone + One>,a b => a.clone().mul(b.clone()),T::one());
    impl<T: Clone + Zero + Neg<Output = T>> Group for Additive<T> {
        fn inverse(a: &Self::S) -> Self::S {
            a.clone().neg()
        }
    }
    impl<T: Clone + Zero> ComMonoid for Additive<T> {}
    impl<T: Clone + Zero + Neg<Output = T>> ComGroup for Additive<T> {}
}

#[test]
fn add_mul() {
    use __traits::SemiGroup;
    type T = Additive<i64>;
    assert_eq!(T::operator(&1, &-200), -199);
}
