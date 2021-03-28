pub use self::math::*;
mod math {
    use rustc_hash::FxHashMap;
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
        let mut ret = Vec::new();
        for i in (1..).take_while(|&i| i * i <= n).filter(|&i| n % i == 0) {
            ret.push(i);
            if i * i < n {
                ret.push(n / i);
            }
        }
        ret
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
    pub fn pf_osa_k(table: &Vec<usize>, mut n: usize) -> FxHashMap<usize,usize> {
        let mut ret = FxHashMap::default();
        while n > 1 {
            //ret.push(table[n]);
            *ret.entry(table[n]).or_insert(0)+=1;
            n /= table[n];
        }
        ret
    }
}

#[test]
fn test() {
    let table = osa_k(1000);
    for i in 0..100 {
        dbg!(pf_osa_k(&table, i));
    }
}
