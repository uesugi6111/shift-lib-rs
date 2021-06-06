pub use self::segment_tree::*;
#[macro_use]
extern crate __traits as traits;
mod segment_tree {
    use std::ops::{Bound, RangeBounds};

    use __general::ceil_pow2;
    use __traits::Monoid;
    // 1-indexed
    pub struct SegmentTree<T: Monoid> {
        n: usize,
        size: usize,
        height: usize,
        data: Vec<T::S>,
    }
    impl<T: Monoid> SegmentTree<T> {
        pub fn new(n: usize) -> Self {
            let height = ceil_pow2(n as u32) as usize;
            let size = 1 << height;
            Self {
                n,
                size,
                height,
                data: vec![T::identity(); 2 * size],
            }
        }
        pub fn apply<F: Fn(&T::S) -> T::S>(&mut self, mut p: usize, f: F) {
            assert!(p < self.n);
            p += self.size;
            self.data[p] = f(&self.data[p]);
            for _ in 0..self.height {
                p >>= 1;
                self.data[p] = T::operator(&self.data[2 * p], &self.data[2 * p + 1]);
            }
        }
        pub fn set(&mut self, p: usize, x: T::S) {
            self.apply(p, |_| x.clone());
        }
        pub fn query<R: RangeBounds<usize>>(&self, range: R) -> T::S {
            let mut l = match range.start_bound() {
                Bound::Unbounded => 0,
                Bound::Excluded(&s) => s + 1,
                Bound::Included(&s) => s,
            } + self.size;
            let mut r = match range.end_bound() {
                Bound::Unbounded => self.data.len(),
                Bound::Excluded(&t) => t,
                Bound::Included(&t) => t + 1,
            } + self.size;
            let mut l_val = T::identity();
            let mut r_val = T::identity();
            while l < r {
                if l & 1 != 0 {
                    l_val = T::operator(&l_val, &self.data[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    r_val = T::operator(&self.data[r], &r_val);
                }
                l >>= 1;
                r >>= 1;
            }
            T::operator(&l_val, &r_val)
        }
        pub fn get(&self, i: usize) -> T::S {
            assert!(i < self.n);
            self.data[i + self.size].clone()
        }
    }
    impl<T: Monoid> From<Vec<T::S>> for SegmentTree<T> {
        fn from(a: Vec<T::S>) -> Self {
            let n = a.len();
            let height = ceil_pow2(n as u32) as usize;
            let size = 1 << height;
            let mut data = vec![T::identity(); 2 * size];
            data[size..(size + n)].clone_from_slice(&a);
            for i in (1..size).rev() {
                data[i] = T::operator(&data[2 * i], &data[2 * i + 1]);
            }
            Self {
                n,
                size,
                height,
                data,
            }
        }
    }
}

#[test]
fn test() {}
