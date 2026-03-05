pub const MASK_ROOK: [u64; 64 as usize] = {
    let mut result: [u64; 64] = [0u64; 64];
    const STEPS: [i8; 4] = [-8, -1, 1, 8];

    let mut sq = 0;
    while sq < 64 {
        let mut i = 0;
        while i < 4 {
            let mut from = sq;
            let mut to = sq + STEPS[i];
            while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                result[sq as usize] |= 1u64 << from;
                from = to;
                to = to + STEPS[i];
            }
            i += 1;
        }
        result[sq as usize] ^= 1u64 << sq;
        sq +=1;
    }
    result
};

pub const MASK_BISHOP: [u64; 64 as usize] = {
    let mut result: [u64; 64] = [0u64; 64];
    const STEPS: [i8; 4] = [-9, -7, 7, 9];

    let mut sq = 0;
    while sq < 64 {
        let mut i = 0;
        while i < 4 {
            let mut from = sq;
            let mut to = sq + STEPS[i];
            while to >= 0 && to < 64 && ((from & 0b111i8) - (to & 0b111i8)).abs() <= 1 {
                result[sq as usize] |= 1u64 << from;
                from = to;
                to = to + STEPS[i];
            }
            i += 1;
        }
        result[sq as usize] ^= 1u64 << sq;
        sq +=1;
    }
    result
};