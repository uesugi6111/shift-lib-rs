pub use self::bitset::*;

mod bitset {

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bitset(usize);
    pub struct BitsetRangeIter {
        start: usize,
        end: usize,
        cur: Option<usize>,
    }

    impl BitsetRangeIter {
        pub fn gen(n: usize) -> Self {
            Bitset((1 << n) - 1).subsets()
        }
    }

    pub struct BitsetRange {
        start: usize,
        end: usize,
    }

    impl BitsetRange {
        fn iter(&self) -> BitsetRangeIter {
            BitsetRangeIter {
                start: self.start,
                end: self.end,
                cur: Some(0),
            }
        }
    }

    impl From<std::ops::RangeInclusive<Bitset>> for BitsetRange {
        fn from(r: std::ops::RangeInclusive<Bitset>) -> Self {
            Self {
                start: r.start().0,
                end: r.end().0,
            }
        }
    }

    pub struct BitsetIter {
        val: usize,
        cur: Option<usize>,
    }

    impl Iterator for BitsetIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.val == 0 {
                None
            } else {
                match self.cur {
                    Some(res) => {
                        self.cur = (!self.val)
                            .checked_add(res << 1)
                            .and_then(|x| Some(x & self.val));
                        Some(res.trailing_zeros() as usize)
                    }
                    None => None,
                }
            }
        }
    }

    impl IntoIterator for Bitset {
        type Item = usize;
        type IntoIter = BitsetIter;
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl Iterator for BitsetRangeIter {
        type Item = Bitset;

        fn next(&mut self) -> Option<Self::Item> {
            match self.cur {
                Some(res) => {
                    self.cur = Some(
                        ((res as i32 - (self.end & (!self.start)) as i32)
                            & (self.end & (!self.start)) as i32) as usize,
                    );
                    if self.cur.unwrap() == 0 {
                        self.cur = None;
                    }
                    Some(Bitset(res | self.start))
                }

                None => None,
            }
        }
    }

    impl IntoIterator for BitsetRange {
        type Item = Bitset;
        type IntoIter = BitsetRangeIter;
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl std::fmt::Debug for Bitset {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.iter().collect::<Vec<_>>().fmt(f)
        }
    }

    impl Bitset {
        pub fn elem(&self, i: usize) -> bool {
            (self.0 >> i) & 1 == 1
        }
        pub fn insert(&mut self, i: usize) {
            self.0 |= 1 << i;
        }

        pub fn empty() -> Self {
            Bitset(0)
        }
        pub fn iter(&self) -> BitsetIter {
            BitsetIter {
                val: self.0,
                cur: Some((self.0 as i32 & (-(self.0 as i32))) as usize),
            }
        }
        pub fn intersection(&self, other: &Self) -> Self {
            Self(self.0 & other.0)
        }
        pub fn union(&self, other: &Self) -> Self {
            Self(self.0 | other.0)
        }
        pub fn xor(&self, other: &Self) -> Self {
            Self(self.0 ^ other.0)
        }
        pub fn diff(&self, other: &Self) -> Self {
            self.xor(other).intersection(self)
        }
        pub fn subsets(&self) -> BitsetRangeIter {
            BitsetRange::from(Bitset::empty()..=*self).iter()
        }
        pub fn gen(n: usize) -> Self {
            Self((1 << n) - 1)
        }
        pub fn singleton(n: usize) -> Self {
            Self(1 << n)
        }
    }

    impl PartialOrd for Bitset {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let x = self.0 & other.0;
            if self.0 == other.0 {
                Some(std::cmp::Ordering::Equal)
            } else if x == self.0 {
                Some(std::cmp::Ordering::Less)
            } else if x == other.0 {
                Some(std::cmp::Ordering::Greater)
            } else {
                None
            }
        }
    }

    impl std::iter::FromIterator<usize> for Bitset {
        fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
            let mut ret = Bitset::empty();
            for i in iter {
                ret.insert(i);
            }
            ret
        }
    }
}
