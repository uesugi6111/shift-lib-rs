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
    pub fn ceil_pow2(n: u32) -> u32 {
        32 - n.saturating_sub(1).leading_zeros()
    }
    pub fn msb(n:u32) -> u32 {
        31 - n.leading_zeros()
    }
    
}

#[test]
fn test() {
    for i in 1..=10 {
        println!("{} {:?}",ceil_pow2(i),i.next_power_of_two())
    }
}
