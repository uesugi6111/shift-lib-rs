pub extern crate __shift_bitset as bitset;
pub extern crate __shift_general as general;
pub extern crate __shift_iter_ext as iter_ext;
pub extern crate __shift_math as math;
pub extern crate __shift_graph as graph;
pub extern crate __shift_data_structure as data_structure;

#[test]
fn math_test() {
    use math::divisors;
    assert_eq!(divisors(16), vec![1, 2, 4, 8, 16]);
    assert_eq!(divisors(15), vec![1, 3, 5, 15]);
}

#[test]
fn iter_ext_test() {
    use iter_ext::IteratorExt;
    let v = (0..5).accumulate().collect::<Vec<_>>();
    assert_eq!(v,vec![0,0,1,3,6,10])
}

#[test]
fn dst_test() {
    use data_structure::disjoint_sparse_table::DisjointSparseTable;
    use alga::general::Additive;
    let dst = DisjointSparseTable::<i32, Additive>::new(vec![5, 2, 1, 4, 9]);
    assert_eq!(dst.query(0..3),8);
    assert_eq!(dst.query(2..5),14);
    assert_eq!(dst.query(3..5),13);
    assert_eq!(dst.query(0..5),21);
}
