use __shift_traits::{Monoid, Multiplicative};
use num_traits::{One, PrimInt, Zero};
use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div, Mul, MulAssign},
};

pub trait Modulus: 'static + Copy + Eq {
    const MOD: u64;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Fp<M: Modulus> {
    val: u64,
    _phantom: PhantomData<fn() -> M>,
}
impl<M: Modulus> Fp<M> {
    pub fn modulus() -> u64 {
        M::MOD
    }
    pub fn raw(val: u64) -> Self {
        Self {
            val,
            _phantom: PhantomData,
        }
    }
    pub fn new<T: PrimInt>(val: T) -> Self {
        Self::raw(val.to_u64().unwrap() % (M::MOD))
    }
}

impl<M: Modulus> Add for Fp<M> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new((self.val + rhs.val) % M::MOD)
    }
}

impl<M: Modulus> AddAssign for Fp<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.val += rhs.val;
        self.val %= M::MOD;
    }
}

impl<M: Modulus> MulAssign for Fp<M> {
    fn mul_assign(&mut self, rhs: Self) {
        self.val *= rhs.val;
        self.val %= M::MOD;
    }
}

impl<M: Modulus> Mul for Fp<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new((self.val * rhs.val) % M::MOD)
    }
}

impl<M: Modulus> One for Fp<M> {
    fn one() -> Self {
        Self::new(1)
    }
}

impl<M: Modulus> Zero for Fp<M> {
    fn zero() -> Self {
        Self::new(0)
    }

    fn is_zero(&self) -> bool {
        self.val == 0
    }
}
impl<M: Modulus> Fp<M> {
    fn inv(&self) -> Self {
        assert!(!self.is_zero());
        Multiplicative::<Fp<M>>::pow(self, M::MOD - 2)
    }
}

impl<M: Modulus> Div for Fp<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Mod1000000007 {}
impl Modulus for Mod1000000007 {
    const MOD: u64 = 1000000007;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Mod998244353 {}
impl Modulus for Mod998244353 {
    const MOD: u64 = 998244353;
}

pub type F1000000007 = Fp<Mod1000000007>;
pub type F998244353 = Fp<Mod998244353>;

#[test]
fn t() {
    type F = F1000000007;
    let v = F::new(2);
    assert_eq!(v.inv().val, 500000004)
}
