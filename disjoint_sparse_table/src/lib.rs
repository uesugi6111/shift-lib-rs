pub use self::disjoint_sparse_table::*;
#[macro_use]
extern crate __traits as traits;
mod disjoint_sparse_table {
    use __traits::SemiGroup;

    extern crate __general as general;

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
            unsafe {
                for i in 1..=n {
                    let mut v = Vec::with_capacity(m);
                    for j in (1 << i..m).step_by(1 << (i + 1)) {
                        v.push(table.get_unchecked(0).get_unchecked(j - 1).clone());
                        for k in 2..=(1 << i) {
                            v.push(T::operator(
                                &table.get_unchecked(0).get_unchecked(j - k),
                                v.last().unwrap(),
                            ))
                        }
                        v.push(table.get_unchecked(0).get_unchecked(j).clone());
                        for k in 1..(1 << i).min(m - j) {
                            v.push(T::operator(
                                &v.last().unwrap(),
                                &table.get_unchecked(0).get_unchecked(j + k),
                            ));
                        }
                    }
                    table.push(v);
                }
            }
            Self { table }
        }
        pub fn query(&self, range: std::ops::Range<usize>) -> T::S {
            let l = range.start;
            let r = range.end - 1;
            unsafe {
                if l == r {
                    self.table.get_unchecked(0).get_unchecked(l).clone()
                } else {
                    let b = general::msb((l ^ r) as u32) as usize;
                    T::operator(
                        &self.table.get_unchecked(b).get_unchecked(l ^ (1 << b) - 1),
                        &self.table.get_unchecked(b).get_unchecked(r),
                    )
                }
            }
        }
    }
}

#[test]
fn test() {
    use __traits::SemiGroup;
    impl_semigroup!(S,i32,a b => a+b);
    let dst = DisjointSparseTable::<S>::new(vec![5, 2, 1, 4, 9]);
    assert_eq!(dst.query(0..3), 8);
    assert_eq!(dst.query(2..5), 14);
    assert_eq!(dst.query(3..5), 13);
    assert_eq!(dst.query(0..5), 21);
    impl_semigroup!(M,i32,a b => *a.min(b));
    let dst = DisjointSparseTable::<M>::new(vec![2, 4, 4, 9, 4, 9]);
    assert_eq!(dst.query(0..3), 2);
}
