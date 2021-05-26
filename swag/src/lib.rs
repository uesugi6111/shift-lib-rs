#[macro_use]
extern crate __shift_traits as traits;
mod swag {
    use __shift_traits::SemiGroup;

    pub struct SWAG<'a, T: SemiGroup> {
        data: std::slice::Iter<'a, T::S>,
        front_stack: Vec<T::S>,
        back_stack: Vec<(T::S, T::S)>,
        left_offset: usize,
        right_offset: usize,
    }
    impl<'a, T: SemiGroup> SWAG<'a, T> {
        pub fn new(a: &'a Vec<<T as SemiGroup>::S>) -> Self {
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
        }
        fn pop(&mut self) {
            if self.front_stack.is_empty() {
                let (x,_) = self.back_stack.pop().unwrap();
                self.front_stack.push(x);
                while let Some((x, _)) = self.back_stack.pop() {
                    self.front_stack
                        .push(T::operator(&x, self.front_stack.last().unwrap()));
                }
            }
            self.front_stack.pop();
        }
        pub fn query(&mut self, l: usize, r: usize) -> T::S {
            assert!(self.left_offset <= l && self.right_offset <= r && l <= r);
            let l = l - self.left_offset;
            let r = r - self.right_offset;
            for _ in 0..r {
                self.push();
            }
            for _ in 0..l {
                self.pop();
            }
            self.left_offset += l;
            self.right_offset += r;
            if self.front_stack.is_empty() {
                self.back_stack.last().unwrap().clone().1
            } else {
                if self.back_stack.is_empty() {
                    self.front_stack.last().unwrap().clone()
                } else {
                    T::operator(
                        self.front_stack.last().unwrap(),
                        &self.back_stack.last().unwrap().1,
                    )
                }
            }
        }
    }
}

#[test]
fn t() {
    use __shift_traits::SemiGroup;
    impl_semigroup!(S,usize,a b => *a.min(&b));
    let n = 7;
    let l = 3;
    let a = vec![1, 7, 7, 4, 8, 1, 6];
    let mut swag = swag::SWAG::<S>::new(&a);
    let ans = (0..n-l+1).map(|i| swag.query(i, i+l)).collect::<Vec<_>>();
    assert_eq!(ans,vec![1,4,4,1,1])
}
