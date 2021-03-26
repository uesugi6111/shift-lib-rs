extern crate alga;
use proconio::input;
use __shift_data_structure::DisjointSparseTable;
use alga::general::AbstractSemigroup;
use alga::general::{AbstractMagma, Additive, Operator};
use __shift_traits::Min;    
fn main() {
    let dst = DisjointSparseTable::<usize,Min>::new(vec![6,3,5]);
    println!("{}",dst.query(0..2));
}
