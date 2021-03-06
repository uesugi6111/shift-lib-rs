pub use self::iter_ext::*;
mod iter_ext {
    pub struct Scanl<I: Iterator, St, F: FnMut(&St, I::Item) -> St> {
        iter: I,
        state: Option<St>,
        f: F,
    }

    pub struct Accumulate<I: Iterator> {
        iter: I,
        state: Option<I::Item>,
    }

    impl<I: Iterator> Accumulate<I>
    where
        I::Item: num_traits::Zero,
    {
        fn new(iter: I) -> Self {
            Self {
                iter,
                state: Some(num_traits::zero()),
            }
        }
    }

    impl<I> Iterator for Accumulate<I>
    where
        I: Iterator,
        I::Item: std::ops::Add<I::Item, Output = I::Item> + Clone,
    {
        type Item = I::Item;
        fn next(&mut self) -> Option<Self::Item> {
            let a = self.state.take()?;
            let ret = a.clone();
            self.state = self.iter.next().map(|x| a + x);
            Some(ret)
        }
    }
    #[derive(Debug)]
    pub struct AccumulateArray<T>(Vec<T>);

    impl<T> AccumulateArray<T>
    where
        T: std::ops::Add + std::ops::Sub + std::ops::Sub<Output = T> + Copy,
    {
        // a_l + a_{l+1} + ... + a_{r-1}
        pub fn get_sum(&self, range: std::ops::Range<usize>) -> T {
            let l = range.start;
            let r = range.end;
            assert!(l <= r);
            self.0[r] - self.0[l]
        }
    }

    impl<I, St, F> Iterator for Scanl<I, St, F>
    where
        I: Iterator,
        St: Clone,
        F: FnMut(&St, I::Item) -> St,
    {
        type Item = St;
        fn next(&mut self) -> Option<Self::Item> {
            let a = self.state.take()?;
            self.state = self.iter.next().map(|x| (self.f)(&a, x));
            Some(a)
        }
    }

    impl<I: Iterator, St: Sized, F: FnMut(&St, I::Item) -> St> Scanl<I, St, F> {
        pub fn new(iter: I, state: St, f: F) -> Scanl<I, St, F> {
            Scanl {
                iter,
                state: Some(state),
                f,
            }
        }
    }

    impl<T> std::iter::FromIterator<T> for AccumulateArray<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            AccumulateArray(iter.into_iter().collect::<Vec<_>>())
        }
    }

    impl<T> IntoIterator for AccumulateArray<T> {
        type Item = T;

        type IntoIter = std::vec::IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            (self.0).into_iter()
        }
    }
    pub trait IteratorExt: Iterator {
        fn scanl<St, F: FnMut(&St, Self::Item) -> St>(
            self,
            initial_state: St,
            f: F,
        ) -> Scanl<Self, St, F>
        where
            Self: Sized,
        {
            Scanl::new(self, initial_state, f)
        }
        fn accumulate(self) -> Accumulate<Self>
        where
            Self: Sized + Iterator,
            Self::Item: std::ops::Add<Self::Item, Output = Self::Item>
                + std::ops::Sub<Self::Item, Output = Self::Item>
                + num_traits::Zero,
        {
            Accumulate::new(self)
        }
    }

    impl<T: Iterator> IteratorExt for T {}
    #[macro_export]
    macro_rules! scanl1 {
        ($a:tt $b:tt => $res:expr,$init:expr,$i:expr) => {
            $i.iter().scanl($init, |$a, $b| $res).collect::<Vec<_>>()
        };
    }
}

#[test]
fn test() {
    let v = (0..5).accumulate().collect::<Vec<_>>();
    assert_eq!(v, vec![0, 0, 1, 3, 6, 10]);
    let v = [5, 2, 1, 4, 9];
    let v = scanl1!(a b => {
        let mut c = a.clone(); 
        c.push_str(&b.to_string()); 
        c.clone()
    },"".to_string(),v);
    println!("{:?}", v);
}
