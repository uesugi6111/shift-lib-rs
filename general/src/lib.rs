pub use self::general::*;

mod general {
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
