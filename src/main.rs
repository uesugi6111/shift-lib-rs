extern crate alga;
use proconio::input;
use alga::general::AbstractSemigroup;
use alga::general::{AbstractMagma, Additive, Operator};
use __shift_traits::Min;    
use __shift_data_structure::disjoint_sparse_table::DisjointSparseTable;
use __shift_general::max_assign;
fn main() {
    let dst = DisjointSparseTable::<usize,Additive>::new(vec![6,3,5]);
    println!("{}",dst.query(0..2));
    
}
