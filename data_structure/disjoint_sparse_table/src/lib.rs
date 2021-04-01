pub use self::disjoint_sparse_table::*;
mod disjoint_sparse_table {
    use __shift_traits::Wrap;
    use alga::general::{AbstractMagma, AbstractSemigroup, Operator};
    use std::marker::PhantomData;
    extern crate __shift_general as general;

    pub struct DisjointSparseTable<T, O>
    where
        O: Operator,
        Wrap<T>: AbstractSemigroup<O>,
    {
        table: Vec<Vec<Wrap<T>>>,
        _phantom: PhantomData<O>,
    }
    impl<T, O> DisjointSparseTable<T, O>
    where
        O: Operator,
        Wrap<T>: AbstractSemigroup<O>,
        T: Clone,
    {
        pub fn new(a: Vec<T>) -> Self {
            let m = a.len();
            let n = general::msb(a.len() as u32) as usize;
            let mut table: Vec<Vec<Wrap<T>>> = Vec::new();
            table.push(a.iter().cloned().map(Wrap).collect());
            for i in 1..=n {
                let mut v = Vec::with_capacity(m);
                for j in (1 << i..m).step_by(1 << (i + 1)) {
                    v.push(table[0][j - 1].clone());
                    for k in 2..=(1 << i) {
                        v.push(table[0][j - k].operate(v.last().unwrap()));
                    }
                    v.push(table[0][j].clone());
                    for k in 1..(1 << i).min(m - j) {
                        v.push(v.last().unwrap().operate(&table[0][j + k]));
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
                self.table[0][l].clone().0
            } else {
                let b = general::msb((l ^ r) as u32) as usize;
                self.table[b][l ^ (1 << b) - 1].operate(&self.table[b][r]).0
            }
        }
    }
    impl<T, O> std::fmt::Debug for DisjointSparseTable<T, O>
    where
        O: Operator,
        Wrap<T>: AbstractSemigroup<O> + Copy + std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.table.fmt(f)
        }
    }
}

#[test]
fn p() {
    //panic!();
}

#[test]
fn test() {
    use alga::general::Additive;
    let dst = DisjointSparseTable::<i32, Additive>::new(vec![5, 2, 1, 4, 9]);
    assert_eq!(dst.query(0..3),8);
    assert_eq!(dst.query(2..5),14);
    assert_eq!(dst.query(3..5),13);
    assert_eq!(dst.query(0..5),21);
}