pub use self::permutations::*;
mod permutations {
    #[derive(Clone, Eq, PartialEq, Debug)]
    pub struct Permutation {
        perm: Vec<usize>,
    }

    impl Permutation {
        pub fn new(perm: &Vec<usize>) -> Self {
            Self { perm: perm.clone() }
        }
        pub fn id(size: usize) -> Self {
            Self {
                perm: (0..size).collect(),
            }
        }
        pub fn comp(&self, other: Self) -> Self {
            let n = self.perm.len();
            let mut ret = vec![0; n];
            for i in 0..n {
                ret[i] = self.perm[other.perm[i]];
            }
            Self { perm: ret }
        }
        pub fn cycle_decomposition(&self) -> Vec<Vec<usize>> {
            let n = self.perm.len();

            let mut seen = vec![false; n];
            let mut ret = Vec::new();
            for i in 0..n {
                if !seen[i] {
                    let mut v = Vec::new();
                    let mut cur = i;
                    while !seen[cur] {
                        v.push(cur);
                        seen[cur] = true;
                        cur = self.perm[cur];
                    }
                    ret.push(v);
                }
            }
            ret
        }
        pub fn pow(&self, k: usize) -> Self {
            let n = self.perm.len();

            let cycles = self.cycle_decomposition();
            let mut par = vec![0; n];
            let mut num = vec![0; n];
            for (i, v) in cycles.iter().enumerate() {
                for (idx, j) in v.iter().enumerate() {
                    par[*j] = i;
                    num[*j] = idx;
                }
            }
            let mut ret = vec![0; n];
            for i in 0..n {
                let j = par[i];
                let s = cycles[j].len();
                ret[i] = cycles[j][(num[i] + k) % s];
            }
            Self { perm: ret }
        }
        pub fn inv(self) -> Self {
            let n = self.perm.len();
            let mut ret = vec![0; n];
            for i in 0..n {
                ret[self.perm[i]] = i;
            }
            Self { perm: ret }
        }
        /// O(n^2)
        pub fn trans_decomp(&self) -> Vec<(usize, usize)> {
            let cycles = self.cycle_decomposition();
            let mut ret = Vec::new();
            for cycle in cycles {
                ret.append(&mut cycle.windows(2).map(|arr| (arr[1], arr[0])).collect());
            }
            ret
        }
        /// O(n^2) たぶん
        pub fn adj_trans_decomp(&self) -> Vec<(usize, usize)> {
            let trans = self.trans_decomp();
            trans.iter().flat_map(|s| Self::trans_to_adj(*s)).collect()
        }
        fn trans_to_adj((mut i, mut j): (usize, usize)) -> Vec<(usize, usize)> {
            if j < i {
                std::mem::swap(&mut i, &mut j);
            }
            if i + 1 == j {
                vec![(i, j)]
            } else {
                (j - 1..=i + 1)
                    .chain(std::iter::once(i))
                    .chain((j - 1..=i + 1).rev())
                    .map(|i| (i, i + 1))
                    .collect::<Vec<_>>()
            }
        }
    }

    impl std::ops::Mul for Permutation {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            self.comp(rhs)
        }
    }
}

#[test]
fn t() {
    let perm = Permutation::new(&vec![0, 3, 2, 1]);
    assert_eq!(perm.adj_trans_decomp(),vec![(2,3),(1,2),(2,3)]);
}
