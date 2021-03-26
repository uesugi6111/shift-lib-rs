pub use self::rangeset::*;
mod rangeset {
    pub struct RangeSet {
        s: std::collections::BTreeSet<(i64, i64)>,
        cnt: usize,
    }

    impl RangeSet {
        pub fn new() -> Self {
            RangeSet {
                s: std::collections::BTreeSet::new(),
                cnt: 0,
            }
        }

        // x 以上であって self に含まれない最小の元を返す
        pub fn mex(&self, x: i64) -> i64 {
            if let Some(&(_, u)) = self.prev((x + 1, x + 1)) {
                u
            } else {
                x
            }
        }
        pub fn insert(&mut self, range: std::ops::Range<i64>) {
            let (mut l, mut r) = (range.start, range.end);
            if l >= r {
                return;
            }
            let mut l1 = std::i64::MIN;
            let mut r1 = std::i64::MIN;
            if let Some(&(_l, _r)) = self.prev((l, r)) {
                l1 = _l;
                r1 = _r;
            }
            if l1 <= l && r <= r1 {
                // [l1..l..r..r1)
                return;
            }
            if l1 <= l && l <= r1 {
                // [l1..l..r1..r)
                l = l1;
                if let Some(&(l2, r2)) = self.next((l1, r1)) {
                    self.s.remove(&(l1, r1));
                    self.cnt -= (r1 - l1) as usize;
                    l1 = l2;
                    r1 = r2;
                };
            } else {
                if let Some(&(l2, r2)) = self.next((l1, r1)) {
                    l1 = l2;
                    r1 = r2;
                } else {
                    l1 = std::i64::MAX;
                    r1 = std::i64::MAX;
                };
            }
            while r > r1 {
                if let Some(&(l2, r2)) = self.next((l1, r1)) {
                    self.s.remove(&(l1, r1));
                    self.cnt -= (r1 - l1) as usize;
                    l1 = l2;
                    r1 = r2;
                } else {
                    self.s.remove(&(l1, r1));
                    self.cnt -= (r1 - l1) as usize;
                    l1 = std::i64::MAX;
                    r1 = std::i64::MAX;
                };
            }
            if l1 <= r {
                self.s.remove(&(l1, r1));
                self.cnt -= (r1 - l1) as usize;
                r = r1;
            }
            self.s.insert((l, r));
            self.cnt += (r - l) as usize;
        }
        pub fn prev(&self, u: (i64, i64)) -> Option<&(i64, i64)> {
            self.s.range(..=u).next_back()
        }
        pub fn next(&self, u: (i64, i64)) -> Option<&(i64, i64)> {
            let mut itr = self.s.range(u..);
            let v = itr.next()?;
            if *v == u {
                itr.next()
            } else {
                Some(v)
            }
        }
        pub fn remove(&mut self, range: std::ops::Range<i64>) {
            let (l, r) = (range.start, range.end);
            if let Some(&(l1, r1)) = self.prev((l, std::i64::MAX)) {
                // l1 <= l
                assert!(r <= r1);
                // [l1..l..r..r1) -> [l1..l) + [r..r1)
                self.s.remove(&(l1, r1));
                if l1 < l {
                    self.s.insert((l1, l));
                }
                if r < r1 {
                    self.s.insert((r, r1));
                }
                self.cnt -= (r - l) as usize;
            };
        }
    }
}
pub use self::disjoint_sparse_table::*;
mod disjoint_sparse_table {
    use alga::general::AbstractSemigroup;
    use alga::general::Operator;
    use std::marker::PhantomData;
    extern crate __shift_general as general;

    pub struct DisjointSparseTable<T, O>
    where
        O: Operator,
        T: AbstractSemigroup<O>,
    {
        table: Vec<Vec<T>>,
        _phantom: PhantomData<O>,
    }
    impl<T, O> DisjointSparseTable<T, O>
    where
        O: Operator,
        T: AbstractSemigroup<O> + Copy,
    {
        pub fn new(a: Vec<T>) -> Self {
            let m = a.len();
            let n = general::msb(a.len() as u32) as usize;
            let mut table = Vec::new();
            table.push(a);
            for i in 1..=n {
                let mut v = Vec::with_capacity(m);
                for j in (1 << i..m).step_by(1 << (i + 1)) {
                    v.push(table[0][j - 1]);
                    for k in 2..=(1 << i) {
                        v.push(T::operate(&table[0][j - k], v.last().unwrap()));
                    }
                    v.push(table[0][j]);
                    for k in 1..(1 << i).min(m - j) {
                        v.push(T::operate(v.last().unwrap(), &table[0][j + k]));
                    }
                }
                table.push(v);
            }
            Self {
                table,
                _phantom: PhantomData,
            }
        }
        pub fn query(&self, range: std::ops::Range<usize>) -> T {
            let l = range.start;
            let r = range.end - 1;
            if l == r {
                self.table[0][l]
            } else {
                let b = general::msb((l ^ r) as u32) as usize;
                T::operate(&self.table[b][l ^ (1 << b) - 1], &self.table[b][r])
            }
        }
    }
    impl<T, O> std::fmt::Debug for DisjointSparseTable<T, O>
    where
        O: Operator,
        T: AbstractSemigroup<O> + Copy + std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.table.fmt(f)
        }
    }
}

#[test]
fn test() {
    use alga::general::{Additive,Multiplicative};
    
    let dst = disjoint_sparse_table::DisjointSparseTable::<i64,Multiplicative>::new(vec![2, 4, 5]);
    assert_eq!(dst.query(1..3), 20)
}
