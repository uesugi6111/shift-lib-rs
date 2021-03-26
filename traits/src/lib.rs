pub use self::traits::*;
#[macro_use]
extern crate alga;

mod traits {
    use alga::general::{AbstractMagma, AbstractSemigroup, Additive, Identity, Operator};
    #[derive(Clone, Copy)]

    pub struct Min;
    #[derive(Clone, Copy)]

    pub struct Max;
    impl Operator for Min {
        #[inline]
        fn operator_token() -> Self {
            Min
        }
    }

    impl Operator for Max {
        #[inline]
        fn operator_token() -> Self {
            Max
        }
    }

    macro_rules! impl_magma(
        ($M:ty; $op: ident; $($T:ty),* $(,)*) => {
            $(impl AbstractMagma<$M> for $T {
                #[inline]
                fn operate(&self, lhs: &Self) -> Self {
                    *self.$op(lhs)
                }
            })*
        });
    macro_rules! impl_marker(
    // Finds the generic parameters of the type and implements the trait for it
    (@para_rec
        [$tra1t:ty, ($($clause:tt)+), ($($type_constr:tt)*)]
        (< $($params:tt)*)
    ) => {
        impl< $($params)* $tra1t for $($type_constr)*< $($params)*
            where $($clause)+
        {}
    };
    // Munches some token trees for searching generic parameters of the type
    (@para_rec
        [$tra1t:ty, ($($clause:tt)+), ($($prev:tt)*)]
        ($cur:tt $($rest:tt)*)
    ) => {
        impl_marker!(@para_rec
            [$tra1t, ($($clause)+), ($($prev)* $cur)]
            ($($rest)*)
        );
    };
    // Handles the trailing separator after where clause
    (@where_rec
        [$tra1t:ty, ($($typ3:tt)+), ($($clause:tt)+)]
        ($(;)*)
    ) => {
        impl_marker!(@para_rec
            [$tra1t, ($($clause)+), ()]
            ($($typ3)+)
        );
    };
    // Implements the trait for the generic type and continues searching other types
    (@where_rec
        [$tra1t:ty, ($($typ3:tt)+), ($($clause:tt)+)]
        (; $($rest:tt)+)
    ) => {
        impl_marker!(@para_rec
            [$tra1t, ($($clause)+), ()]
            ($($typ3)+)
        );
        impl_marker!(@rec
            [$tra1t, ()]
            ($($rest)+)
        );
    };
    // Munches some token trees for searching the end of the where clause
    (@where_rec
        [$tra1t:ty, ($($typ3:tt)+), ($($prev:tt)*)]
        ($cur:tt $($rest:tt)*)
    ) => {
        impl_marker!(@where_rec
            [$tra1t, ($($typ3)+), ($($prev)* $cur)]
            ($($rest)*)
        );
    };
    // Handles the trailing separator for non-generic type and implements the trait
    (@rec
        [$tra1t:ty, ($($typ3:tt)*)]
        ($(;)*)
    ) => {
        impl $tra1t for $($typ3)* { }
    };
    // Implements the trait for the non-generic type and continues searching other types
    (@rec
        [$tra1t:ty, ($($typ3:tt)*)]
        (; $($rest:tt)+)
    ) => {
        impl $tra1t for $($typ3)* { }
        impl_marker!(@rec
            [$tra1t, ()]
            ($($rest)+)
        );
    };
    // Detects that there is indeed a where clause for the type and tries to find where it ends.
    (@rec
        [$tra1t:ty, ($($prev:tt)+)]
        (where $($rest:tt)+)
    ) => {
        impl_marker!(@where_rec
            [$tra1t, ($($prev)+), ()]
            ($($rest)+)
        );
    };
    // Munches some token trees for detecting if we have where clause or not
    (@rec
        [$tra1t:ty, ($($prev:tt)*)]
        ($cur:tt $($rest:tt)*)
    ) => {
        impl_marker!(@rec
            [$tra1t, ($($prev)* $cur)]
            ($($rest)*)
        );
    };
    // Entry point to the macro
    ($tra1t:ty; $($rest:tt)+) => {
        impl_marker!(@rec
            [$tra1t, ()]
            ($($rest)+)
        );
    };
);
    macro_rules! impl_semigroup(
    (<$M:ty> for $($T:tt)+) => {
        impl_marker!(alga::general::AbstractSemigroup<$M>; $($T)+);
    });
    macro_rules! impl_monoid(
        (<$M:ty> for $($T:tt)+) => {
            impl_semigroup!(<$M> for $($T)+);
            impl_marker!(alga::general::AbstractMonoid<$M>; $($T)+);
        }
    );
    macro_rules! impl_ident {
    ($M:ty; $V:expr; $($T:ty),* $(,)*) => {
        $(impl Identity<$M> for $T { #[inline] fn identity() -> $T {$V} })+
    }
    }
    impl_ident!(Min;0;u8, u16, u32, u64, u128, usize);
    impl_magma!(Min;min;u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
    impl_magma!(Max;max;u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
    impl_semigroup!(<Min> for i8;i16;i32;i64;i128;isize);
    impl_semigroup!(<Max> for u8; u16; u32; u64; u128; usize;i8;i16;i32;i64;i128;isize);
    impl_monoid!(<Min> for u8; u16; u32; u64; u128; usize);
}

#[test]
fn t() {
    1.min(2);
}
