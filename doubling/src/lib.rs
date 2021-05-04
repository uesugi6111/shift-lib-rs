pub use self::doubling::*;

#[macro_use]
extern crate __shift_traits as traits;

mod doubling {
    use std::{collections::HashMap, hash::Hash};
    use traits::{Monoid,SemiGroup};
    #[derive(Clone,Debug,PartialEq, Eq)]
    pub struct Mapping(Vec<usize>);
    #[derive(Clone,Debug)]
    pub enum MappingI {
        Identity,
        F(Mapping)
    }
    impl PartialEq for MappingI {
        fn eq(&self, other: &Self) -> bool {
            match (self,other) {
                (MappingI::Identity, MappingI::Identity) => {true}
                (MappingI::Identity, MappingI::F(o)) => { let m = o.0.len(); (0..m).collect::<Vec<_>>() == o.0}
                (MappingI::F(s), MappingI::Identity) => {let m = s.0.len(); (0..m).collect::<Vec<_>>() == s.0}
                (MappingI::F(s), MappingI::F(o)) => {s == o}
            }
    }
    }
    impl Eq for MappingI {
        
    }
    #[derive(Clone)]
    pub struct HashMapping<T: Hash + Eq>(HashMap<T, T>);
    #[derive(Clone)]
    pub enum HashMappingI<T : Hash + Eq> {
        Identity,
        F(HashMapping<T>)
    }
    impl Mapping {
        pub fn new(a:&Vec<usize>)->Self {
            Self(a.clone())
        }
    }
    impl MappingI {
        pub fn from_func<F: Fn(usize) -> usize>(f: F, m: usize) -> Self {
            Self::F(Mapping((0..m).map(|x| f(x)).collect()))
        }
        pub fn new(a:&Vec<usize>)->Self {
            Self::F(Mapping(a.clone()))
        }
        pub fn composition(&self,rhs:&Self)->Self {
            match (self,rhs) {
                (Self::Identity,r) => r.clone(),
                (s,Self::Identity) => s.clone(),
                (Self::F(f),Self::F(g)) => {
                    assert_eq!(f.0.len(),g.0.len());
                    let m = f.0.len();
                    Self::F(Mapping((0..m).map(|x| f.0[g.0[x]]).collect()))
                },
                _ => unreachable!()
            }
        }
    }
    impl_monoid!(MappingI,f g => f.composition(g),MappingI::Identity);
}
#[test]
fn t() {
    use __shift_traits::Monoid;
    let f = MappingI::from_func(|x| (2*x)%5,5);
    let ff = f.composition(&f);
    let a = MappingI::pow(&f, 2);
    assert_eq!(ff,MappingI::F(Mapping::new(&vec![0,4,3,2,1])));
    assert_eq!(ff,a);
    let a = MappingI::pow(&f, 10000000);
    println!("{:?}",a);
    assert_eq!(a,MappingI::Identity);
}
