pub use self::swag::*;
mod swag {
    use std::ops::{Bound, RangeBounds};

    use __traits::SemiGroup;

    pub struct SWAG<'a, T: SemiGroup> {
        data: std::slice::Iter<'a, T::S>,
        front_stack: Vec<T::S>,
        back_stack: Vec<(T::S, T::S)>,
        left_offset: usize,
        right_offset: usize,
    }
    impl<'a, T: SemiGroup> SWAG<'a, T> {
        pub fn new(a: &'a [T::S]) -> Self {
            SWAG {
                data: a.iter(),
                front_stack: Vec::new(),
                back_stack: Vec::new(),
                left_offset: 0,
                right_offset: 0,
            }
        }
        fn push(&mut self) {
            let val = self.data.next().unwrap();
            if self.back_stack.is_empty() {
                self.back_stack.push((val.clone(), val.clone()));
            } else {
                self.back_stack.push((
                    val.clone(),
                    T::operator(&self.back_stack.last().unwrap().1, val),
                ))
            }
            self.right_offset += 1;
        }
        fn pop(&mut self) {
            if self.front_stack.is_empty() {
                let (x, _) = self.back_stack.pop().unwrap();
                self.front_stack.push(x);
                while let Some((x, _)) = self.back_stack.pop() {
                    self.front_stack
                        .push(T::operator(&x, self.front_stack.last().unwrap()));
                }
            }
            self.front_stack.pop();
            self.left_offset += 1;
        }
        pub fn query<R:RangeBounds<usize>>(&mut self,range:R) -> T::S {
            let l = match range.start_bound() {
                Bound::Unbounded => 0,
                Bound::Excluded(&s) => s + 1,
                Bound::Included(&s) => s,
            };
            let r = match range.end_bound() {
                Bound::Unbounded => self.data.len(),
                Bound::Excluded(&t) => t,
                Bound::Included(&t) => t + 1,
            } ;
            assert!(self.left_offset <= l && self.right_offset <= r && l <= r);
            let l = l - self.left_offset;
            let r = r - self.right_offset;
            for _ in 0..r {
                self.push();
            }
            for _ in 0..l {
                self.pop();
            }
            match (self.front_stack.last(), self.back_stack.last()) {
                (Some(f_last), Some((_, b_last))) => T::operator(f_last, b_last),
                (Some(f_last), _) => f_last.clone(),
                (_, Some((_, b_last))) => b_last.clone(),
                _ => unreachable!(),
            }
        }
    }
}
