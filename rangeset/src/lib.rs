pub use self::rangeset::*;
use std::collections::BTreeSet;
mod rangeset {
    pub struct RangeSet {
        s: BTreeSet<(i64, i64)>,
        cnt: usize,
    }
    
    impl RangeSet {
        pub fn new() -> Self {
            RangeSet {
                s: BTreeSet::new(),
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
        pub fn insert(&mut self, range: Range<i64>) {
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
        pub fn remove(&mut self, range: Range<i64>) {
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