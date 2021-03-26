pub use self::graph::*;
#[macro_use]
mod graph {
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap, VecDeque},
        hash::{BuildHasherDefault, Hash},
        ops::Sub,
    };

    use num_traits::{One, Zero};
    use rustc_hash::{FxHashMap, FxHasher};

    pub struct GeneralGraph<'a, I, T, Itr: IntoIterator<Item = (I, T)>> {
        next: &'a mut dyn FnMut(I, T) -> Itr,
    }

    #[derive(Copy, Clone)]
    struct FirstOrd<S: Ord, T>(S, T);

    impl<S: PartialEq + Ord, T> PartialEq for FirstOrd<S, T> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl<S: Eq + Ord, T> Eq for FirstOrd<S, T> {}

    impl<S: Ord + PartialEq, T> PartialOrd for FirstOrd<S, T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl<S: Ord + Eq, T> Ord for FirstOrd<S, T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl<T: Zero + Ord + Copy, I: Hash + Copy + Eq, Itr: IntoIterator<Item = (I, T)>>
        GeneralGraph<'_, I, T, Itr>
    {
        pub fn dijkstra(self, start: I) -> HashMap<I, T, BuildHasherDefault<FxHasher>> {
            let mut queue = BinaryHeap::new();
            queue.push(Reverse(FirstOrd(T::zero(), start)));
            let mut dist = FxHashMap::default();
            dist.insert(start, T::zero());
            while !queue.is_empty() {
                let Reverse(FirstOrd(cost, idx)) = queue.pop().unwrap();
                if dist.get(&idx).and_then(|&x| Some(x < cost)) == Some(true) {
                    continue;
                }
                for (idx2, next_cost) in (self.next)(idx, cost) {
                    assert!(next_cost >= cost);
                    if dist.get(&idx2).and_then(|&x| Some(x <= next_cost)) == Some(true) {
                        continue;
                    }
                    dist.insert(idx2, next_cost);
                    queue.push(Reverse(FirstOrd(next_cost, idx2)));
                }
            }
            dist
        }
    }

    impl<
            T: Copy + Zero + Ord + One + Sub<Output = T>,
            I: Hash + Eq + Copy,
            Itr: IntoIterator<Item = (I, T)>,
        > GeneralGraph<'_, I, T, Itr>
    {
        pub fn bfs_01(self, start: I) -> HashMap<I, T, BuildHasherDefault<FxHasher>> {
            let mut queue = VecDeque::new();
            queue.push_back((T::zero(), start));
            let mut dist = FxHashMap::default();
            dist.insert(start, T::zero());
            while !queue.is_empty() {
                let (cost, idx) = queue.pop_front().unwrap();
                if dist.get(&idx).and_then(|&x| Some(x < cost)) == Some(true) {
                    continue;
                }
                for (idx2, next_cost) in (self.next)(idx, cost) {
                    assert!(next_cost - cost == T::zero() || next_cost - cost == T::one());
                    if dist.get(&idx2).and_then(|&x| Some(x <= next_cost)) == Some(true) {
                        continue;
                    }
                    dist.insert(idx2, next_cost);
                    if next_cost == cost {
                        queue.push_front((next_cost, idx2));
                    } else {
                        queue.push_back((next_cost, idx2));
                    }
                }
            }
            dist
        }
        pub fn floyd_warshall(
            self,
            vertices: Vec<I>,
        ) -> HashMap<(I, I), T, BuildHasherDefault<FxHasher>> {
            let mut dist = FxHashMap::default();
            for &i in &vertices {
                for (j, cost) in (self.next)(i.into(), T::zero()) {
                    //dist[i][j.into()] = Some(cost);
                    dist.insert((i, j), cost);
                }
            }
            for &k in &vertices {
                for &i in &vertices {
                    for &j in &vertices {
                        if let (Some(&dik), Some(&dkj)) = (dist.get(&(i, k)), dist.get(&(k, j))) {
                            if dist.get(&(i, j)).is_none()
                                || *dist.get(&(i, j)).unwrap() > dik + dkj
                            {
                                dist.insert((i, j), dik + dkj);
                            }
                        }
                    }
                }
            }
            dist
        }
    }

    impl<'a, T, I, Itr: IntoIterator<Item = (I, T)>> GeneralGraph<'a, I, T, Itr> {
        pub fn new(next: &'a mut dyn FnMut(I, T) -> Itr) -> Self {
            GeneralGraph { next }
        }
    }

    macro_rules! from_adj_matrix {
        ($adj:expr,$cost_type:ty) => {
            GeneralGraph {
                next: &mut (move |idx: usize, cost: $cost_type| -> Vec<_> {
                    ($adj)[idx]
                        .iter()
                        .enumerate()
                        .map(|(i, &v)| if v == 1 { Some((i, 1 + cost)) } else { None })
                        .flatten()
                        .collect()
                }),
            }
        };
    }

    macro_rules! from_edges {
        ($edges:expr,$cost_type:ty) => {
            GeneralGraph {
                next: &mut (|idx: usize, cost: $cost_type| -> Vec<_> {
                    $edges[idx].iter().map(|&(i, c)| (i, c + cost)).collect()
                }),
            }
        };
    }
}

#[test]
fn test() {
    let mut v = vec![1, 2, 3, 4, 5];
}
