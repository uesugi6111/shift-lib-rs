pub use self::disjoint_sparse_table::*;
#[macro_use]
extern crate __shift_traits as traits;
mod disjoint_sparse_table {
    use __shift_traits::SemiGroup;


    extern crate __shift_general as general;

    pub struct DisjointSparseTable<T>
    where
        T: SemiGroup,
    {
        table: Vec<Vec<T::S>>,
    }
    impl<T> DisjointSparseTable<T>
    where
        T: SemiGroup + Clone,
    {
        pub fn new(a: Vec<T::S>) -> Self {
            let m = a.len();
            let n = general::msb(a.len() as u32) as usize;
            let mut table: Vec<Vec<T::S>> = Vec::new();
            table.push(a.iter().cloned().collect());
            for i in 1..=n {
                let mut v = Vec::with_capacity(m);
                for j in (1 << i..m).step_by(1 << (i + 1)) {
                    v.push(table[0][j - 1].clone());
                    for k in 2..=(1 << i) {
                        v.push(T::operator(&table[0][j - k], v.last().unwrap()));
                    }
                    v.push(table[0][j].clone());
                    for k in 1..(1 << i).min(m - j) {
                        v.push(T::operator(&v.last().unwrap(), &table[0][j + k]));
                    }
                }
                table.push(v);
            }
            Self { table }
        }
        pub fn query(&self, range: std::ops::Range<usize>) -> T::S {
            let l = range.start;
            let r = range.end - 1;
            if l == r {
                self.table[0][l].clone()
            } else {
                let b = general::msb((l ^ r) as u32) as usize;
                T::operator(&self.table[b][l ^ (1 << b) - 1], &self.table[b][r])
            }
        }
    }
}


#[test]
fn test() {
    use __shift_traits::SemiGroup;
    impl_semigroup!(S,i32,a b => a+b);
    let dst = DisjointSparseTable::<S>::new(vec![5, 2, 1, 4, 9]);
    assert_eq!(dst.query(0..3),8);
    assert_eq!(dst.query(2..5),14);
    assert_eq!(dst.query(3..5),13);
    assert_eq!(dst.query(0..5),21);
}
