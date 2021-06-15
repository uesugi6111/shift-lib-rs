fn main() {}

#[test]
fn t() {
    //panic!();
    let mut v = vec![5,2,1,4,9];
    v.windows(2).step_by(2);
}
