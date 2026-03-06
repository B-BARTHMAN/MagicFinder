pub fn slider<const t: u8>(sq: i8, blockers: u64) -> u64 {
    let mut bb = 0u64;
    let steps: [i8; 4] = {
        match t {
            0 => [-9, -7, 7, 9], // BISHOP
            1 => [-8, -1, 1, 8], // ROOK
            _ => [127, 127, 127, 127],
        }
    };
    
    for step in steps.iter() {
        let mut from = sq;
        let mut to = sq + *step;
        'inner: while to >= 0 && to < 64 && ((from & 0b111) - (to & 0b111)).abs() <= 1{
            let to_bb = 1u64 << to;
            bb |= to_bb;
            if to_bb & blockers != 0 { break 'inner; }
            from = to;
            to += *step;
        }
    }
    
    bb
}