pub use self::algebraic_structures_impl::*;
mod algebraic_structures_impl {
    use num_traits::{Zero,Bounded,One};
    use __traits::*;
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
fn add_mul() {
    use __traits::SemiGroup;
    type T = Additive<i64>;
    assert_eq!(T::operator(&1, &-200),-199);
}