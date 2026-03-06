pub fn blockers(mut mask: u64, mut i: usize) -> u64 {
    let mut bb = 0u64;
    while mask != 0 {
        let lsb = mask.trailing_zeros();
        bb |= ((i & 1) as u64) << lsb;
        mask &= mask -1;
        i >>= 1;
    }
    bb
}