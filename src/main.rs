use std::{array, thread};
use std::fs::File;
use std::io::Write;
use rand::random;
use crate::blocker::blockers;
use crate::mask::{BITS_BISHOP, BITS_ROOK, MASK_BISHOP, MASK_ROOK};
use crate::slider::slider;

mod mask;
mod bits;
mod blocker;
mod slider;

fn main() {
  // BISHOP
  thread::spawn(||{
    let mut bishop_blockers : [Vec<u64>; 64] = array::from_fn(|i| Vec::with_capacity(1 << BITS_BISHOP[i]));
    let mut bishop_slider   : [Vec<u64>; 64] = array::from_fn(|i| Vec::with_capacity(1 << BITS_BISHOP[i]));

    for sq in 0..64{
      for i in 0..(1 << BITS_BISHOP[sq]) {
        bishop_blockers[sq].push(blockers(MASK_BISHOP[sq], i));
        bishop_slider[sq].push(slider::<0>(sq as i8, bishop_blockers[sq][i]));
      }
    }

    let mut bits: [Option<u8>;64] = [None; 64];
    let mut magics: [Option<u64>;64] = [None; 64];

    loop {
      let mut skipped: u8 = 0;
      'square: for sq in 0..64 {
        if let Some(b) = bits[sq] && b == BITS_BISHOP[sq] - 1 {
          skipped += 1;
          if skipped == 64 {
            return;
          }
          continue 'square;
        }
        let bit: u8 = bits[sq].map_or(BITS_BISHOP[sq] + 1, |b| b - 1);
        'magic: for _ in 0..1_000_000{
          let magic = random::<u64>();

          if (magic.wrapping_mul(MASK_BISHOP[sq]) & 0xFF00000000000000).count_ones() < 6 {
            continue;
          }

          let mut attacks: Vec<Option<u64>> = vec![None; 1 << bit];
          for i in 0..(1 << BITS_BISHOP[sq]) {
            let index = (magic.wrapping_mul(bishop_blockers[sq][i]) >> (64 - bit)) as usize;
            if let Some(b) = attacks[index as usize] && b != bishop_slider[sq][i] {
              continue 'magic;
            }
            attacks[index] = Some(bishop_slider[sq][i]);
          }
          magics[sq] = Some(magic);
          bits[sq] = Some(bit);
          break 'magic;
        }
      }
      if magics.iter().all(|m| m.is_some()) {
        let mut file = File::create("magic_bishop.txt").expect("Unable to create file");
        writeln!(file, "Magics:").unwrap();
        for m in magics.iter() {
          writeln!(file, "{:?}", m.unwrap()).unwrap();
        }
        writeln!(file, "\nBits:").unwrap();
        for s in bits.iter() {
          writeln!(file, "{:?}", s.unwrap()).unwrap();
        }
      }

    }
  });
  // ROOK
  thread::spawn(||{
    let mut rook_blockers : [Vec<u64>; 64] = array::from_fn(|i| Vec::with_capacity(1 << BITS_ROOK[i]));
    let mut rook_slider   : [Vec<u64>; 64] = array::from_fn(|i| Vec::with_capacity(1 << BITS_ROOK[i]));

    for sq in 0..64{
      for i in 0..(1 << BITS_ROOK[sq]) {
        rook_blockers[sq].push(blockers(MASK_ROOK[sq], i));
        rook_slider[sq].push(slider::<1>(sq as i8, rook_blockers[sq][i]));
      }
    }

    let mut bits: [Option<u8>;64] = [None; 64];
    let mut magics: [Option<u64>;64] = [None; 64];

    loop {
      let mut skipped: u8 = 0;
      'square: for sq in 0..64 {
        if let Some(b) = bits[sq] && b == BITS_ROOK[sq] - 1 {
          skipped += 1;
          if skipped == 64 {
            return;
          }
          continue 'square;
        }
        let bit: u8 = bits[sq].map_or(BITS_ROOK[sq] + 1, |b| b - 1);
        'magic: for _ in 0..1_000_000{
          let magic = random::<u64>();

          if (magic.wrapping_mul(MASK_ROOK[sq]) & 0xFF00000000000000).count_ones() < 6 {
            continue;
          }

          let mut attacks: Vec<Option<u64>> = vec![None; 1 << bit];
          for i in 0..(1 << BITS_ROOK[sq]) {
            let index = (magic.wrapping_mul(rook_blockers[sq][i]) >> (64 - bit)) as usize;
            if let Some(b) = attacks[index as usize] && b != rook_slider[sq][i] {
              continue 'magic;
            }
            attacks[index] = Some(rook_slider[sq][i]);
          }
          magics[sq] = Some(magic);
          bits[sq] = Some(bit);
          break 'magic;
        }
      }
      if magics.iter().all(|m| m.is_some()) {
        let mut file = File::create("magic_rook.txt").expect("Unable to create file");
        writeln!(file, "Magics:").unwrap();
        for m in magics.iter() {
          writeln!(file, "{:?}", m.unwrap()).unwrap();
        }
        writeln!(file, "\nBits:").unwrap();
        for s in bits.iter() {
          writeln!(file, "{:?}", s.unwrap()).unwrap();
        }
      }

    }
  });
}