use crate::fp::{Fp,F998244353 as F};

type Poly = Vec<F>;
fn rem(f : &Poly,c:u64) -> Poly {
    todo!()
}
fn fft(f : &Poly,i:u64,flag : bool) -> Poly
// x^n - c を状態に持ってる
{
    let deg = f.len();
    if deg == 0 {
        return f.clone();
    }
    let l = rem(f,i/2);
    let r = rem(f,i/2 ); 
    todo!()
}
