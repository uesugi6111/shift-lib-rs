pub use self::math::*;
pub mod fp;
mod math {
    use rustc_hash::FxHashMap;
    use num_traits;
    pub fn prime_factorization(n: usize) -> FxHashMap<usize, usize> {
        let mut mp = FxHashMap::default();
        let mut tmp = n;
        for i in (2..).take_while(|i| i * i <= n).filter(|i| n % i == 0) {
            let mut cnt = 0;
            while tmp % i == 0 {
                tmp /= i;
                cnt += 1;
            }
            if cnt > 0 {
                mp.insert(i, cnt);
            }
        }
        if tmp != 1 {
            mp.insert(tmp, 1);
        }
        mp
    }
    pub fn divisors(n: usize) -> Vec<usize> {
        let mut ret_l = Vec::new();
        let mut ret_r = Vec::new();
        for i in (1..).take_while(|&i| i * i <= n).filter(|&i| n % i == 0) {
            ret_l.push(i);
            if i * i < n {
                ret_r.push(n / i);
            }
        }
        ret_l.iter().chain(ret_r.iter().rev()).copied().collect()
    }
    pub fn eratosthenes(n: usize) -> Vec<bool> {
        let mut table = vec![true; n + 1];
        table[0] = false;
        table[1] = false;
        for i in (2..).take_while(|&i| i * i <= n) {
            if table[i] {
                for j in (2 * i..).step_by(i).take_while(|&j| j <= n) {
                    table[j] = false;
                }
            }
        }
        table
    }
    pub fn primes(n: usize) -> Vec<usize> {
        let table = eratosthenes(n);
        (0..=n).filter(|x| table[*x]).collect()
    }
    pub fn osa_k(n: usize) -> Vec<usize> {
        let mut table = (0..=n).collect::<Vec<_>>();
        for i in (2..).take_while(|&i| i * i <= n) {
            if table[i] >= i {
                for j in (i * i..).step_by(i).take_while(|&j| j <= n) {
                    if table[j] == j {
                        table[j] = i;
                    }
                }
            }
        }
        table
    }
    pub fn pf_osa_k(table: &Vec<usize>, mut n: usize) -> FxHashMap<usize, usize> {
        let mut ret = FxHashMap::default();
        while n > 1 {
            //ret.push(table[n]);
            *ret.entry(table[n]).or_insert(0) += 1;
            n /= table[n];
        }
        ret
    }
    use acl_modint::{StaticModInt,Modulus};
    use __shift_iter_ext::IteratorExt;
    pub struct FpUtils<M> {
        fact_ : Vec<StaticModInt<M>>,
        inv_fact_ : Vec<StaticModInt<M>>
    }
    impl<M:Modulus> FpUtils<M> {
        pub fn new(n:usize) -> Self {
            let fact_ = 
            (1..=n).map(StaticModInt::new).scanl(StaticModInt::new(1), |x,y|x*y).collect::<Vec<_>>();
            let mut inv_fact_ = vec![StaticModInt::new(0);n+1];
            inv_fact_[n]=StaticModInt::new(1)/fact_[n];
            for i in (0..n).rev() {
                inv_fact_[i] = inv_fact_[i+1]*StaticModInt::new(i+1);
            }
            Self {fact_,inv_fact_}
        }
        pub fn fact(&self,n:usize) -> StaticModInt<M> {
            self.fact_[n]
        }
        pub fn inv_fact(&self,n:usize) -> StaticModInt<M> {
            self.inv_fact_[n]
        }
        pub fn binom(&self,n:usize,r:usize) -> StaticModInt<M> {
            assert!(r <= n);
            self.fact_[n]*self.inv_fact_[r]*self.inv_fact_[n-r]
        }
    }

}

#[test]
fn test() {
    assert_eq!(divisors(16), vec![1, 2, 4, 8, 16]);
    assert_eq!(divisors(15), vec![1, 3, 5, 15]);
}
