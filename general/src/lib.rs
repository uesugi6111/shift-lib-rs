pub use self::general::*;

mod general {
    use std::{
        io::empty,
        ops::{Range, RangeBounds},
    };

    #[macro_export]
    macro_rules! max_assign {
        ($left:expr,$right:expr) => {
            if $left < $right {
                $left = $right;
            }
        };
    }
    #[macro_export]
    macro_rules! min_assign {
        ($left:expr,$right:expr) => {
            if $left > $right {
                $left = $right;
            }
        };
    }
    pub fn ceil_pow2(n: u32) -> u32 {
        32 - n.saturating_sub(1).leading_zeros()
    }
    pub fn msb(n: u32) -> u32 {
        31 - n.leading_zeros()
    }
    pub fn print_lines<I: IntoIterator>(a: I)
    where
        I::Item: std::fmt::Display,
    {
        let a = a.into_iter();
        for v in a {
            println!("{}", v);
        }
    }
    pub fn print_oneline<I: IntoIterator>(a: I)
    where
        I::Item: std::fmt::Display,
    {
        println!(
            "{}",
            a.into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
    pub fn binary_search<R: RangeBounds<i64>, F: Fn(i64) -> bool>(range: R, f: F) -> Range<i64> {
    let mut ok = match range.start_bound() {
        std::ops::Bound::Included(l) => *l,
        std::ops::Bound::Excluded(l) => l + 1,
        std::ops::Bound::Unbounded => std::i64::MIN,
    };
    let mut ng = match range.end_bound() {
        std::ops::Bound::Included(r) => r + 1,
        std::ops::Bound::Excluded(r) => *r,
        std::ops::Bound::Unbounded => std::i64::MAX,
    };
    if !f(ok) && !f(ng) {
        return 0..0;
    }
    if !f(ok) {
        std::mem::swap(&mut ok, &mut ng);
    }
    while (ok > ng && ok - ng > 1) || (ok < ng && ng - ok > 1) {
        let mid = if ok > ng {
            ok + (ng - ok) / 2
        } else {
            ng + (ok - ng) / 2
        };
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    if ok < ng {
        ok..ng
    } else {
        ng + 1..ok + 1
    }
}
}

#[test]
fn test_output() {
    let v = [5, 2, 1, 4, 9];
    let vv = vec![5, 2, 1, 4, 9];
    let null: Vec<i32> = Vec::new();
    print_lines(&null);
    print_oneline(&null);
    print_lines(&vv);
    print_oneline(&vv);
    println!("a");
}
