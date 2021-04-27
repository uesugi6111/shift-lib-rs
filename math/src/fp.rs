mod fp {
    #[derive(Clone, Copy,PartialEq, Eq,Debug)]
    pub struct Fp<const MOD:usize> {
        val:u64,
        m:u64
    }
    
}