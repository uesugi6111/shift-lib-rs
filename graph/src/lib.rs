pub use self::graph::*;
pub mod grid;
#[macro_use]
mod graph {
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap, VecDeque},
        hash::{BuildHasherDefault, Hash},
        ops::Add,
    };

    use num_traits::{One, Zero};
    use rustc_hash::{FxHashMap, FxHasher};

    pub struct GeneralGraph<'a, I, T, C, Itr: IntoIterator<Item = (I, T, C)>> {
        pub next: &'a mut dyn Fn(I, C) -> Itr,
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

    impl<
            'a,
            T: Zero + Ord + Copy,
            I: Hash + Copy + Eq,
            Itr: IntoIterator<Item = (I, T, C)>,
            C: Hash + Eq + Copy,
        > GeneralGraph<'a, I, T, C, Itr>
    {
        pub fn dijkstra(
            &self,
            start: I,
            init: C,
        ) -> HashMap<(I, C), T, BuildHasherDefault<FxHasher>> {
            let mut queue = BinaryHeap::new();
            queue.push(Reverse(FirstOrd(T::zero(), (start, init))));
            let mut dist = FxHashMap::default();
            dist.insert((start, init), T::zero());
            while !queue.is_empty() {
                let Reverse(FirstOrd(cost, (idx, context))) = queue.pop().unwrap();
                if dist.get(&(idx, context)).and_then(|&x| Some(x < cost)) == Some(true) {
                    continue;
                }
                for (idx2, c, next_context) in (self.next)(idx, context) {
                    let next_cost = cost + c;
                    if dist
                        .get(&(idx2, next_context))
                        .and_then(|&x| Some(x <= next_cost))
                        == Some(true)
                    {
                        continue;
                    }
                    dist.insert((idx2, next_context), next_cost);
                    queue.push(Reverse(FirstOrd(next_cost, (idx2, next_context))));
                }
            }
            dist
        }
    }

    impl<
            T: Copy + Zero + Ord + Add<Output = T> + One,
            I: Hash + Eq + Copy,
            C: Hash + Eq + Copy,
            Itr: IntoIterator<Item = (I, T, C)>,
        > GeneralGraph<'_, I, T, C, Itr>
    {
        pub fn bfs_01(
            &self,
            start: I,
            init: C,
        ) -> HashMap<(I, C), T, BuildHasherDefault<FxHasher>> {
            let mut queue = VecDeque::new();
            queue.push_back(Reverse(FirstOrd(T::zero(), (start, init))));
            let mut dist = FxHashMap::default();
            dist.insert((start, init), T::zero());
            while !queue.is_empty() {
                let Reverse(FirstOrd(cost, (idx, context))) = queue.pop_front().unwrap();
                if dist.get(&(idx, context)).and_then(|&x| Some(x < cost)) == Some(true) {
                    continue;
                }
                for (idx2, c, next_context) in (self.next)(idx, context) {
                    let next_cost = cost + c;
                    if dist
                        .get(&(idx2, next_context))
                        .and_then(|&x| Some(x <= next_cost))
                        == Some(true)
                    {
                        continue;
                    }
                    dist.insert((idx2, next_context), next_cost);
                    if c == T::zero() {
                        queue.push_front(Reverse(FirstOrd(next_cost, (idx2, next_context))));
                    } else if c == T::one() {
                        queue.push_back(Reverse(FirstOrd(next_cost, (idx2, next_context))));
                    } else {
                        panic!("Cost is only 0 or 1")
                    }
                }
            }
            dist
        }
    }
    impl<
            T: Copy + Zero + Ord,
            I: Hash + Eq + Copy,
            C: Hash + Eq + Copy,
            Itr: IntoIterator<Item = (I, T, C)>,
        > GeneralGraph<'_, I, T, C, Itr>
    {
        pub fn floyd_warshall(
            &self,
            vertices: Vec<(I, C)>,
        ) -> HashMap<((I, C), (I, C)), T, BuildHasherDefault<FxHasher>> {
            let mut dist = FxHashMap::default();
            for &(i, c) in &vertices {
                for (j, cost, ctx) in (self.next)(i, c) {
                    //dist[i][j.into()] = Some(cost);
                    dist.insert(((i, c), (j, ctx)), cost);
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
    impl<'a, T, I, C, Itr: IntoIterator<Item = (I, T, C)>> GeneralGraph<'a, I, T, C, Itr> {
        pub fn new(next: &'a mut dyn Fn(I, C) -> Itr) -> Self {
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
        ($edges:expr) => {{
            GeneralGraph {
                next: &mut |idx: usize, _: ()| -> Vec<_> {
                    ($edges)[idx].iter().map(|&(i, c)| (i, c, ())).collect()
                },
            }
        }};
    }
}
#[test]
fn test() {
    let edges = {
        let mut e = vec![Vec::new(); 3];
        e[0].push((1, 1));
        e[1].push((2, 1));
        e[2].push((0, 1));
        e
    };
    let g = from_edges!(edges);
    let res = g
        .dijkstra(1, ())
        .iter()
        .map(|(&(a, _), &b)| (a, b))
        .collect::<Vec<_>>();
    let res2 = g
        .bfs_01(1, ())
        .iter()
        .map(|(&(a, _), &b)| (a, b))
        .collect::<Vec<_>>();
    assert_eq!(res, vec![(0, 2), (1, 0), (2, 1)]);
    assert_eq!(res, res2);
}
