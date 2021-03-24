pub use self::general::*;
#[macro_use]
mod general {
    macro_rules! max_assign {
        ($left:expr,$right:expr) => {
            if $left < $right {
                $left = $right;
            }
        };
    }
    macro_rules! min_assign {
        ($left:expr,$right:expr) => {
            if $left > $right {
                $left = $right;
            }
        };
    }
}

#[test]
fn test() {
    let mut v = vec![1, 2, 3, 4, 5];
}
