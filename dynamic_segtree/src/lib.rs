pub use self::dynamic_segtree::*;

mod dynamic_segtree {
    use std::ops::{Bound, RangeBounds};

    use __traits::Monoid;
    struct Node<T: Monoid> {
        value: T::S,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>,
    }
    impl<T: Monoid> Node<T> {
        fn new(value: T::S) -> Self {
            Self {
                value,
                left: None,
                right: None,
            }
        }
    }
    pub struct DynamicSegtree<T: Monoid> {
        size: usize,
        root: Option<Box<Node<T>>>,
    }
    
    impl<T: Monoid> DynamicSegtree<T> {
        pub fn new(size: usize) -> Self {
            DynamicSegtree { root: None, size }
        }
        pub fn set(&mut self, idx: usize, value: &T::S) {
            Self::_set(&mut self.root, 0, self.size, idx, value);
        }
        fn _set(root: &mut Option<Box<Node<T>>>, a: usize, b: usize, idx: usize, value: &T::S) {
            if root.is_none() {
                *root = Some(Box::new(Node::new(T::identity())));
            }
            let root = root.as_mut().unwrap();
            if b - a == 1 {
                root.value = value.clone();
            } else {
                let mid = (a + b) / 2;
                if idx < mid {
                    Self::_set(&mut root.left, a, mid, idx, value);
                } else {
                    Self::_set(&mut root.right, mid, b, idx, value);
                }
                let left = &root.as_ref().left;
                let right = &root.as_ref().right;
                root.value = match (left, right) {
                    (None, None) => T::identity(),
                    (None, Some(n)) => n.value.clone(),
                    (Some(m), None) => m.value.clone(),
                    (Some(m), Some(n)) => T::operator(&m.value, &n.value),
                }
            }
        }
        pub fn query<R: RangeBounds<usize>>(&self, range: R) -> T::S {
            let l = match range.start_bound() {
                Bound::Unbounded => 0,
                Bound::Excluded(&s) => s + 1,
                Bound::Included(&s) => s,
            };
            let r = match range.end_bound() {
                Bound::Unbounded => self.size,
                Bound::Excluded(&t) => t,
                Bound::Included(&t) => t + 1,
            };
            Self::prod(&self.root, 0, self.size, l, r)
        }
        fn prod(root: &Option<Box<Node<T>>>, a: usize, b: usize, l: usize, r: usize) -> T::S {
            if root.is_none() || b <= l || r <= a {
                T::identity()
            } else if l <= a && b <= r {
                root.as_ref().unwrap().value.clone()
            } else {
                let root = root.as_ref().unwrap();
                let mid = (a + b) / 2;
                T::operator(
                    &Self::prod(&root.left, a, mid, l, r),
                    &Self::prod(&root.right, mid, b, l, r),
                )
            }
        }
    }
}
