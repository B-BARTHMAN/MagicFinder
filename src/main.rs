use crate::mask::MASK_ROOK;

mod mask;


fn main() {
    for x in MASK_ROOK {
        println!("{:064b}", x);
    }
}